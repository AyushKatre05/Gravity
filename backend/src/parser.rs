use std::path::Path;
use anyhow::{Context, Result};
use walkdir::WalkDir;
use regex::Regex;
use crate::models::{ParsedFile, ParsedFunction};

pub fn parse_directory(root_path: &str) -> Result<Vec<ParsedFile>> {
    let mut results = Vec::new();
    
    for entry in WalkDir::new(root_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|x| x == "rs").unwrap_or(false))
    {
        let path = entry.path();
        match parse_file(path) {
            Ok(pf) => results.push(pf),
            Err(e) => {
                tracing::warn!("Skipping {:?}: {}", path, e);
            }
        }
    }

    Ok(results)
}

fn parse_file(path: &Path) -> Result<ParsedFile> {
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("Cannot read {}", path.display()))?;

    let line_count = source.lines().count();
    let module_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(str::to_owned);

    let (functions, imports, structs) = parse_content(&source, path);

    let path_str = path
        .to_str()
        .unwrap_or_default()
        .replace('\\', "/")
        .to_owned();

    Ok(ParsedFile {
        path: path_str,
        module_name,
        line_count: line_count as usize,
        functions,
        imports,
        structs,
    })
}

fn parse_content(source: &str, _path: &Path) -> (Vec<ParsedFunction>, Vec<String>, Vec<String>) {
    let mut functions = Vec::new();
    let mut imports = Vec::new();
    let mut structs = Vec::new();

    // Extract use/import statements
    extract_imports(source, &mut imports);

    // Extract function definitions
    extract_functions(source, &mut functions);

    // Extract struct definitions
    extract_structs(source, &mut structs);

    (functions, imports, structs)
}

fn extract_imports(source: &str, imports: &mut Vec<String>) {
    let re = Regex::new(r"^\s*(?:pub\s+)?use\s+[^;]+;").unwrap();
    for line in source.lines() {
        if let Some(mat) = re.find(line) {
            imports.push(mat.as_str().trim().to_owned());
        }
    }
}

fn extract_functions(source: &str, functions: &mut Vec<ParsedFunction>) {
    // Match function definitions: [pub] [async] fn name(...)
    let re = Regex::new(
        r"(?m)^\s*(?:pub\s+)?(?:async\s+)?fn\s+([a-zA-Z_]\w*)\s*\(",
    ).unwrap();

    let mut current_line = 0;
    for line in source.lines() {
        current_line += 1;

        if let Some(caps) = re.captures(line) {
            let name = caps.get(1).map(|m| m.as_str().to_owned()).unwrap_or_default();
            let is_public = line.contains("pub fn");
            let is_async = line.contains("async fn");

            // Try to find the function body
            let line_start = current_line;
            let mut line_end = current_line;

            // Scan for closing brace (simplified - counts braces)
            let mut brace_count = 0;
            let mut in_body = false;
            let mut found_opening = false;

            for (i, body_line) in source.lines().skip(line_start - 1).enumerate() {
                brace_count += body_line.matches('{').count() as i32;
                brace_count -= body_line.matches('}').count() as i32;

                if brace_count > 0 {
                    found_opening = true;
                    in_body = true;
                }

                if in_body && brace_count == 0 && found_opening {
                    line_end = line_start + i;
                    break;
                }
            }

            let body_source = source
                .lines()
                .skip(line_start - 1)
                .take(line_end - line_start + 1)
                .collect::<Vec<_>>()
                .join("\n");

            functions.push(ParsedFunction {
                name,
                line_start,
                line_end,
                is_public,
                is_async,
                body_source,
            });
        }
    }
}

fn extract_structs(source: &str, structs: &mut Vec<String>) {
    // Match struct definitions: struct name { ... }
    let re = Regex::new(r"(?m)^\s*(?:pub\s+)?struct\s+([a-zA-Z_]\w*)").unwrap();

    for line in source.lines() {
        if let Some(caps) = re.captures(line) {
            if let Some(name) = caps.get(1) {
                structs.push(name.as_str().to_owned());
            }
        }
    }
}
