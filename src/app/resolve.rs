use crate::domain::normalized::NormalizedFile;
use std::collections::HashMap;

use std::collections::HashSet;

pub fn resolve_symbols(files: &mut [NormalizedFile]) {
    for file in files {
        let mut aliases: HashMap<String, String> = HashMap::new();

        // 1. Collect aliases
        for import in &file.imports {
            if let Some(alias) = &import.alias {
                if let Some(symbol) = &import.symbol {
                    aliases.insert(alias.clone(), symbol.clone());
                } else {
                    // Module alias, e.g. import google.adk as adk
                    aliases.insert(alias.clone(), import.module.clone());
                }
            }
        }

        // 2. Resolve calls
        for call in &mut file.calls {
            // Very basic resolution: if callee matches an alias exactly, replace it.
            // (In a real implementation, we'd handle dotted paths like `adk.Agent` as well)
            if let Some(resolved) = aliases.get(&call.callee) {
                call.callee = resolved.clone();
            } else if let Some((base, rest)) = call.callee.split_once('.')
                && let Some(resolved_base) = aliases.get(base)
            {
                call.callee = format!("{}.{}", resolved_base, rest);
            }
        }
    }
}

pub fn resolve_constants(files: &mut [NormalizedFile]) {
    for file in files.iter_mut() {
        let mut assignments = std::collections::HashMap::new();
        for assignment in &file.assignments {
            assignments.insert(assignment.name.clone(), assignment.value.clone());
        }

        for call in &mut file.calls {
            for (arg_name, arg_value) in &call.keyword_arguments {
                if let Some(resolved_val) = assignments.get(arg_value) {
                    if resolved_val == "True" && !call.true_keywords.contains(arg_name) {
                        call.true_keywords.push(arg_name.clone());
                    }
                    // For static controls, we might want to evaluate it here.
                    // E.g., if resolved_val is a string literal "dontAsk" and arg_name is "permissionMode"
                    // But for now, we just handle booleans as requested by the test.
                    // We can also propagate string constants to `static_controls`!
                    if resolved_val.starts_with('"')
                        || resolved_val.starts_with('\'')
                        || resolved_val
                            .chars()
                            .all(|c| c.is_alphanumeric() || c == '_')
                    {
                        let stripped = resolved_val.trim_matches(|c| c == '"' || c == '\'');
                        let control = format!("{}={}", arg_name, stripped);
                        if !call.static_controls.contains(&control) {
                            call.static_controls.push(control);
                        }
                    }
                }
            }
        }
    }
}

pub fn resolve_wrappers(files: &mut [NormalizedFile], max_depth: usize) {
    for file in files {
        let mut fn_calls: HashMap<String, Vec<crate::domain::normalized::CallFact>> =
            HashMap::new();
        for call in &file.calls {
            if let Some(enclosing) = &call.enclosing_function {
                fn_calls
                    .entry(enclosing.clone())
                    .or_default()
                    .push(call.clone());
            }
        }

        let mut expanded_calls = Vec::new();
        for call in &file.calls {
            let mut visited = HashSet::new();
            expand_call(
                call,
                call.enclosing_function.clone(),
                &fn_calls,
                &mut expanded_calls,
                &mut visited,
                max_depth,
                0,
            );
        }
        file.calls = expanded_calls;
    }
}

fn expand_call(
    call: &crate::domain::normalized::CallFact,
    target_enclosing: Option<String>,
    fn_calls: &HashMap<String, Vec<crate::domain::normalized::CallFact>>,
    expanded: &mut Vec<crate::domain::normalized::CallFact>,
    visited: &mut HashSet<String>,
    max_depth: usize,
    depth: usize,
) {
    let mut current_call = call.clone();
    current_call.enclosing_function = target_enclosing.clone();
    expanded.push(current_call.clone());

    if depth >= max_depth {
        return;
    }

    if visited.contains(&call.callee) {
        return;
    }

    visited.insert(call.callee.clone());

    if let Some(child_calls) = fn_calls.get(&call.callee) {
        for child in child_calls {
            let mut merged_child = child.clone();
            for c in &call.static_controls {
                if !merged_child.static_controls.contains(c) {
                    merged_child.static_controls.push(c.clone());
                }
            }
            expand_call(
                &merged_child,
                target_enclosing.clone(),
                fn_calls,
                expanded,
                visited,
                max_depth,
                depth + 1,
            );
        }
    }

    visited.remove(&call.callee);
}
