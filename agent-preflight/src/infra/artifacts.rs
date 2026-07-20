use std::fs;
use std::path::Path;

pub fn write_all(root: &Path, artifacts: [(&str, String); 3]) -> Result<(), ArtifactError> {
    let output = root.join(".agent-preflight");
    fs::create_dir_all(&output).map_err(|_| ArtifactError::CreateDirectory)?;
    if fs::symlink_metadata(&output)
        .map_err(|_| ArtifactError::CreateDirectory)?
        .file_type()
        .is_symlink()
    {
        return Err(ArtifactError::UnsafeOutputDirectory);
    }
    for (name, content) in artifacts {
        let destination = output.join(name);
        let temporary = output.join(format!(".{name}.tmp"));
        fs::write(&temporary, content).map_err(|_| ArtifactError::Write)?;
        fs::rename(temporary, destination).map_err(|_| ArtifactError::Write)?;
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ArtifactError {
    #[error("could not create the artifact directory")]
    CreateDirectory,
    #[error("artifact directory must not be a symlink")]
    UnsafeOutputDirectory,
    #[error("could not atomically write scan artifacts")]
    Write,
}
