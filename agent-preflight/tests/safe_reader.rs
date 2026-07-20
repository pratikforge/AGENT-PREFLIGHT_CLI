use std::fs;

use agent_preflight::infra::safe_reader::{ReaderError, SafeReader};
use tempfile::tempdir;

#[test]
fn returns_supported_sources_in_lexical_relative_path_order_and_skips_secrets() {
    let repo = tempdir().expect("temporary repository");
    fs::create_dir_all(repo.path().join("nested")).expect("nested directory");
    fs::write(
        repo.path().join("nested\\agent.py"),
        "print('parser input')",
    )
    .expect("python source");
    fs::write(repo.path().join("app.ts"), "export const app = true;").expect("typescript source");
    fs::write(repo.path().join(".env"), "TOKEN=secret").expect("secret file");

    let sources = SafeReader
        .read(repo.path())
        .expect("normal sources should read");
    let paths: Vec<_> = sources.iter().map(|source| source.path.as_str()).collect();

    assert_eq!(paths, ["app.ts", "nested/agent.py"]);
    assert!(
        sources
            .iter()
            .all(|source| !source.content.contains("secret"))
    );
}

#[test]
fn rejects_non_directory_roots_and_invalid_utf8_sources() {
    let repo = tempdir().expect("temporary repository");
    let plain_file = repo.path().join("not-a-root.txt");
    fs::write(&plain_file, "not a directory").expect("plain file");
    assert!(SafeReader.read(&plain_file).is_err());

    fs::write(repo.path().join("broken.py"), [0xff, 0xfe, 0xfd]).expect("invalid UTF-8 source");
    assert_eq!(SafeReader.read(repo.path()), Err(ReaderError::InvalidUtf8));
}

#[test]
fn rejects_oversized_sources_before_parsing() {
    let repo = tempdir().expect("temporary repository");
    fs::write(repo.path().join("too-large.py"), vec![b'x'; 1_048_577]).expect("oversized source");

    assert_eq!(SafeReader.read(repo.path()), Err(ReaderError::FileTooLarge));
}

#[test]
fn ignores_reserved_directories_and_secret_file_patterns() {
    let repo = tempdir().expect("temporary repository");
    for directory in [".git", "node_modules", "target", "dist", ".agent-preflight"] {
        let ignored = repo.path().join(directory);
        fs::create_dir_all(&ignored).expect("ignored directory");
        fs::write(ignored.join("ignored.ts"), "export const leaked = true;")
            .expect("ignored source");
    }
    fs::write(repo.path().join(".env.production"), "TOKEN=secret").expect("secret env");
    fs::write(repo.path().join("certificate.pem"), "private key").expect("pem file");
    fs::write(repo.path().join("credentials.key"), "private key").expect("key file");
    fs::write(repo.path().join("kept.py"), "print('kept')").expect("kept source");

    let sources = SafeReader
        .read(repo.path())
        .expect("reserved paths must be skipped");

    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].path, "kept.py");
}

#[test]
fn rejects_sources_beyond_the_depth_limit() {
    let repo = tempdir().expect("temporary repository");
    let mut nested = repo.path().to_path_buf();
    for index in 0..33 {
        nested.push(format!("level-{index}"));
    }
    fs::create_dir_all(&nested).expect("deep directory");
    fs::write(nested.join("deep.py"), "print('too deep')").expect("deep source");

    assert_eq!(
        SafeReader.read(repo.path()),
        Err(ReaderError::DepthExceeded)
    );
}

#[test]
fn rejects_a_symlinked_source_without_reading_its_target() {
    let repo = tempdir().expect("temporary repository");
    let outside = tempdir().expect("outside directory");
    let target = outside.path().join("outside.py");
    fs::write(&target, "print('outside root')").expect("outside source");
    let link = repo.path().join("linked.py");

    if create_file_symlink(&target, &link).is_err() {
        return;
    }

    assert_eq!(
        SafeReader.read(repo.path()),
        Err(ReaderError::SymlinkRejected)
    );
}

#[test]
fn rejects_a_symlinked_scan_root() {
    let parent = tempdir().expect("temporary parent directory");
    let real_repo = parent.path().join("real-repository");
    fs::create_dir_all(&real_repo).expect("real repository");
    fs::write(real_repo.join("app.py"), "print('inside')").expect("source file");
    let link = parent.path().join("linked-repository");

    if create_directory_symlink(&real_repo, &link).is_err() {
        return;
    }

    assert_eq!(SafeReader.read(&link), Err(ReaderError::SymlinkRejected));
}

#[cfg(unix)]
fn create_file_symlink(target: &std::path::Path, link: &std::path::Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(unix)]
fn create_directory_symlink(
    target: &std::path::Path,
    link: &std::path::Path,
) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_file_symlink(target: &std::path::Path, link: &std::path::Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}

#[cfg(windows)]
fn create_directory_symlink(
    target: &std::path::Path,
    link: &std::path::Path,
) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}
