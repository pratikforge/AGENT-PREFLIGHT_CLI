use crate::domain::normalized::NormalizedFile;
use std::collections::HashMap;

pub struct SymbolResolver<'a> {
    // path -> (alias -> Vec<original_symbol>)
    aliases: HashMap<&'a str, HashMap<&'a str, Vec<&'a str>>>,
}

impl<'a> SymbolResolver<'a> {
    pub fn new(files: &'a [NormalizedFile]) -> Self {
        let mut aliases = HashMap::new();

        for file in files {
            let mut file_aliases: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
            for import in &file.imports {
                if let (Some(alias), Some(symbol)) = (&import.alias, &import.symbol) {
                    file_aliases
                        .entry(alias.as_str())
                        .or_default()
                        .push(symbol.as_str());
                }
            }
            aliases.insert(file.path.as_str(), file_aliases);
        }

        Self { aliases }
    }

    pub fn resolve_alias(&self, path: &str, alias: &str) -> Option<String> {
        let file_aliases = self.aliases.get(path)?;
        let symbols = file_aliases.get(alias)?;
        if symbols.len() == 1 {
            Some(symbols[0].to_string())
        } else {
            None // Shadowed or ambiguous
        }
    }
}

use std::collections::HashSet;

pub struct CallGraphAnalyzer<'a> {
    _files: &'a [NormalizedFile],
    wrappers: std::cell::RefCell<HashMap<String, String>>,
}

impl<'a> CallGraphAnalyzer<'a> {
    pub fn new(files: &'a [NormalizedFile]) -> Self {
        Self {
            _files: files,
            wrappers: std::cell::RefCell::new(HashMap::new()),
        }
    }

    pub fn register_wrapper(&self, from: &str, to: &str) {
        self.wrappers
            .borrow_mut()
            .insert(from.to_string(), to.to_string());
    }

    pub fn resolve_call(&self, _path: &str, callee: &str, max_depth: usize) -> Option<String> {
        let mut current = callee.to_string();
        let mut depth = 0;
        let mut visited = HashSet::new();

        while let Some(target) = self.wrappers.borrow().get(&current) {
            if depth >= max_depth {
                return None;
            }
            if !visited.insert(current.clone()) {
                return None; // Cycle
            }
            current = target.clone();
            depth += 1;
        }

        if depth > 0 { Some(current) } else { None }
    }
}
