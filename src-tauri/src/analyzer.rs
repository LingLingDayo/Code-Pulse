use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use walkdir::WalkDir;

// js/ts: import { ... } from "..." | import "..." | require("...")
static JS_RE: OnceLock<Regex> = OnceLock::new();
// python: import ... | from ... import ...
static PY_RE: OnceLock<Regex> = OnceLock::new();
// rust: use ...; | mod ...;
static RS_RE: OnceLock<Regex> = OnceLock::new();

fn get_js_re() -> &'static Regex {
    JS_RE.get_or_init(|| {
        Regex::new(r#"(?:import.*from\s+['"]([^'"]+)['"]|require\(['"]([^'"]+)['"]\)|import\s+['"]([^'"]+)['"])"#).unwrap()
    })
}

fn get_py_re() -> &'static Regex {
    PY_RE.get_or_init(|| {
        Regex::new(r#"^\s*(?:import|from)\s+([a-zA-Z0-9_\.]+)"#).unwrap()
    })
}

fn get_rs_re() -> &'static Regex {
    RS_RE.get_or_init(|| {
        Regex::new(r#"^\s*(?:use|mod)\s+([a-zA-Z0-9_:]+)"#).unwrap()
    })
}

fn extract_dependencies(content: &str, ext: &str) -> Vec<String> {
    let mut deps = Vec::new();
    match ext {
        "js" | "jsx" | "ts" | "tsx" | "vue" | "svelte" => {
            let re = get_js_re();
            for cap in re.captures_iter(content) {
                if let Some(m) = cap.get(1).or(cap.get(2)).or(cap.get(3)) {
                    deps.push(m.as_str().to_string());
                }
            }
        }
        "py" => {
            let re = get_py_re();
            for cap in re.captures_iter(&content.replace("\r\n", "\n")) {
                if let Some(m) = cap.get(1) {
                    deps.push(m.as_str().replace('.', "/"));
                }
            }
        }
        "rs" => {
            let re = get_rs_re();
            for cap in re.captures_iter(&content.replace("\r\n", "\n")) {
                if let Some(m) = cap.get(1) {
                    deps.push(m.as_str().replace("::", "/"));
                }
            }
        }
        _ => {}
    }
    deps
}

fn resolve_path(base_dir: &Path, import_path: &str, ext: &str) -> Option<PathBuf> {
    if !import_path.starts_with(".") && !import_path.starts_with("/") {
        return None;
    }
    
    let target = base_dir.join(import_path);
    if target.exists() && target.is_file() {
        return Some(target);
    }
    
    let extensions = match ext {
        "js" | "jsx" | "ts" | "tsx" | "vue" => vec!["ts", "js", "tsx", "jsx", "vue"],
        "py" => vec!["py"],
        "rs" => vec!["rs"],
        _ => vec![],
    };

    for e in &extensions {
        let with_ext = target.with_extension(e);
        if with_ext.exists() {
            return Some(with_ext);
        }
    }
    
    if target.is_dir() {
        for e in &extensions {
            let index_path = target.join(format!("index.{}", e));
            if index_path.exists() {
                return Some(index_path);
            }
        }
    }

    None
}

fn find_project_root(start_path: &Path) -> PathBuf {
    let mut current = start_path;
    loop {
        if current.join("package.json").exists() || current.join("Cargo.toml").exists() || current.join(".git").exists() {
            if let Ok(canon) = current.canonicalize() {
                return canon;
            }
        }
        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            break;
        }
    }
    
    let fallback = if start_path.is_dir() {
        start_path.to_path_buf()
    } else {
        start_path.parent().unwrap_or(Path::new("")).to_path_buf()
    };
    if let Ok(canon) = fallback.canonicalize() {
        canon
    } else {
        fallback
    }
}

fn parse_ignore_patterns(raw: &str, defaults: Vec<String>) -> (HashSet<String>, HashSet<String>, HashSet<String>, Vec<Regex>) {
    let mut names = HashSet::new();
    let mut exts = HashSet::new();
    let mut fnames = HashSet::new();
    let mut regexes = Vec::new();

    let mut all_patterns = defaults;
    if !raw.is_empty() {
        for p in raw.split(|c| c == ',' || c == '\n' || c == '\r') {
            let s = p.trim().to_string();
            if !s.is_empty() {
                all_patterns.push(s);
            }
        }
    }

    for s in all_patterns {
        if s.contains('*') {
            let mut escaped = regex::escape(&s);
            escaped = escaped.replace("\\*", ".*");
            let pattern = format!("^{}$", escaped);
            if let Ok(re) = Regex::new(&pattern) {
                regexes.push(re);
            }
        } else if s.starts_with('.') {
            exts.insert(s.to_lowercase());
        } else if s.contains('.') {
            fnames.insert(s);
        } else {
            names.insert(s);
        }
    }
    (names, exts, fnames, regexes)
}

pub fn analyze_dependencies(paths: Vec<String>, max_depth: usize, generate_tree: bool, ignore_exts: String, ignore_deep_parse: String) -> Result<String, String> {
    let mut visited: HashSet<PathBuf> = HashSet::new();
    let mut result_blocks: Vec<String> = Vec::new();
    let mut parsed_paths: Vec<String> = Vec::new();

    let supported_exts = vec!["js", "jsx", "ts", "tsx", "vue", "svelte", "py", "rs"];

    let ignores_defaults: Vec<String> = vec![
        "node_modules", ".git", "dist", "target", "build", ".vscode", ".idea", 
        ".next", ".nuxt", ".output", ".vercel", ".github", 
        "*.lock", "*-lock.json", "package-lock.json", "yarn.lock", "pnpm-lock.yaml",
        ".jpg", ".jpeg", ".png", ".gif", ".svg", ".ico", ".webp",
        ".mp4", ".avi", ".mkv", ".mov", ".webm",
        ".mp3", ".wav", ".flac", ".aac", ".ogg",
        ".zip", ".tar", ".gz", ".7z", ".rar",
        ".exe", ".dll", ".so", ".dylib",
        ".log", ".tmp", ".temp", ".swp", ".DS_Store"
    ].into_iter().map(|s| s.to_string()).collect();

    let (ignore_names, ignore_extensions, ignore_filenames, ignore_regexes) = 
        parse_ignore_patterns(&ignore_exts, ignores_defaults);

    let (ignore_deep_names, ignore_deep_extensions, ignore_deep_filenames, ignore_deep_regexes) = 
        parse_ignore_patterns(&ignore_deep_parse, vec![]);

    for p_str in paths {
        let path = Path::new(&p_str);
        if !path.exists() { continue; }

        let base_path = find_project_root(path);

        if path.is_dir() {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                let e_path = entry.path();
                if e_path.is_file() {
                    let ext = e_path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    if supported_exts.contains(&ext) {
                        if should_ignore(e_path, &ignore_names, &ignore_extensions, &ignore_filenames, &ignore_regexes) {
                            continue;
                        }
                        process_file(e_path, 0, max_depth, &mut visited, &mut result_blocks, &mut parsed_paths, &base_path, 
                            &ignore_names, &ignore_extensions, &ignore_filenames, &ignore_regexes,
                            &ignore_deep_names, &ignore_deep_extensions, &ignore_deep_filenames, &ignore_deep_regexes);
                    }
                }
            }
        } else {
            process_file(path, 0, max_depth, &mut visited, &mut result_blocks, &mut parsed_paths, &base_path, 
                &ignore_names, &ignore_extensions, &ignore_filenames, &ignore_regexes,
                &ignore_deep_names, &ignore_deep_extensions, &ignore_deep_filenames, &ignore_deep_regexes);
        }
    }

    let mut final_blocks = Vec::new();
    if generate_tree {
        let tree_str = build_file_tree(&parsed_paths);
        final_blocks.push(tree_str);
    }
    final_blocks.extend(result_blocks);

    Ok(final_blocks.join("\n\n"))
}

fn build_file_tree(paths: &[String]) -> String {
    let mut tree = String::from("========================================\n[FILE TREE]\n========================================\n.\n");
    let mut sorted_paths = paths.to_vec();
    sorted_paths.sort();
    
    let mut prev_components: Vec<String> = Vec::new();
    for path in &sorted_paths {
        let components: Vec<String> = path.split('/').map(|s| s.to_string()).collect();
        let mut i = 0;
        while i < components.len() && i < prev_components.len() && components[i] == prev_components[i] {
            i += 1;
        }
        while i < components.len() {
            let indent = "│   ".repeat(i);
            tree.push_str(&format!("{}├── {}\n", indent, components[i]));
            i += 1;
        }
        prev_components = components;
    }
    tree
}

fn should_ignore(
    path: &Path, 
    ignore_names: &HashSet<String>, 
    ignore_extensions: &HashSet<String>, 
    ignore_filenames: &HashSet<String>,
    ignore_regexes: &[Regex]
) -> bool {
    let fname = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    let fname_lower = fname.to_lowercase();

    // 1. Check dot-prefixed patterns (suffix match)
    for ext in ignore_extensions {
        if fname_lower.ends_with(ext) {
            return true;
        }
    }

    // 2. Check full filename match
    if ignore_filenames.contains(fname) {
        return true;
    }

    // 3. Check regexes against filename
    for re in ignore_regexes {
        if re.is_match(fname) {
            return true;
        }
    }

    // 4. Check each component for patterns (directory/file match)
    for component in path.components() {
        if let Some(comp_str) = component.as_os_str().to_str() {
            if ignore_names.contains(comp_str) {
                return true;
            }
            for re in ignore_regexes {
                if re.is_match(comp_str) {
                    return true;
                }
            }
        }
    }

    false
}

fn process_file(
    path: &Path, 
    current_depth: usize, 
    max_depth: usize, 
    visited: &mut HashSet<PathBuf>, 
    result_blocks: &mut Vec<String>,
    parsed_paths: &mut Vec<String>,
    base_path: &Path,
    ignore_names: &HashSet<String>,
    ignore_extensions: &HashSet<String>,
    ignore_filenames: &HashSet<String>,
    ignore_regexes: &[Regex],
    ignore_deep_names: &HashSet<String>,
    ignore_deep_extensions: &HashSet<String>,
    ignore_deep_filenames: &HashSet<String>,
    ignore_deep_regexes: &[Regex]
) {
    if current_depth > max_depth || !path.exists() { return; }
    
    let abs_path = match path.canonicalize() { Ok(p) => p, Err(_) => return };
    if visited.contains(&abs_path) || abs_path.as_os_str().is_empty() { return; }
    
    if should_ignore(&abs_path, ignore_names, ignore_extensions, ignore_filenames, ignore_regexes) {
        return;
    }
    
    visited.insert(abs_path.clone());
    
    if let Ok(content) = fs::read_to_string(&abs_path) {
        let mut display_path_str = abs_path.to_string_lossy().into_owned();
        if let Ok(rel_path) = abs_path.strip_prefix(base_path) {
            display_path_str = rel_path.to_string_lossy().replace("\\", "/");
        } else {
            display_path_str = display_path_str.replace("\\", "/");
            // Also try to strip UNC prefix if present
            if display_path_str.starts_with("//?/") {
                display_path_str = display_path_str[4..].to_string();
            }
        }

        parsed_paths.push(display_path_str.clone());

        result_blocks.push(format!(
            "========================================\n[FILE PATH]: {}\n(Dependency Layer: {})\n========================================\n[CONTENT START]\n{}\n[CONTENT END]", 
            display_path_str, current_depth, content
        ));
        
        let ext = abs_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        
        if !should_ignore(&abs_path, ignore_deep_names, ignore_deep_extensions, ignore_deep_filenames, ignore_deep_regexes) {
            let base_dir = abs_path.parent().unwrap_or(Path::new(""));
            for dep in extract_dependencies(&content, ext) {
                if let Some(resolved) = resolve_path(base_dir, &dep, ext) {
                    process_file(&resolved, current_depth + 1, max_depth, visited, result_blocks, parsed_paths, base_path, 
                        ignore_names, ignore_extensions, ignore_filenames, ignore_regexes,
                        ignore_deep_names, ignore_deep_extensions, ignore_deep_filenames, ignore_deep_regexes);
                }
            }
        }
    }
}
