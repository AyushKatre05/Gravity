use regex::Regex;
use anyhow::Result;

use crate::models::ParsedFunction;

pub fn compute_complexity(func: &ParsedFunction) -> Result<usize> {
    // Simple regex-based complexity counting
    // Count branch constructs in the function body
    let mut count = 0usize;

    let source = &func.body_source;

    // Count if expressions
    if Regex::new(r"\bif\s*[({]").unwrap().is_match(source) {
        count += 1;
    }

    // Count else if
    count += Regex::new(r"\belse\s+if\s*[({]").unwrap()
        .find_iter(source)
        .count();

    // Count match arms
    count += Regex::new(r"^\s*\w+\s*=>").unwrap()
        .find_iter(source)
        .count()
        .saturating_sub(1); // subtract 1 because match itself adds baseline

    // Count for loops
    count += Regex::new(r"\bfor\s+\w+\s+in\b").unwrap()
        .find_iter(source)
        .count();

    // Count while loops
    count += Regex::new(r"\bwhile\s*[({]").unwrap()
        .find_iter(source)
        .count();

    // Count loop constructs
    count += Regex::new(r"\bloop\s*[({]").unwrap()
        .find_iter(source)
        .count();

    // Count try expressions (? operator)
    count += Regex::new(r"\?\s*[,;}\)]").unwrap()
        .find_iter(source)
        .count();

    // Count closures
    count += Regex::new(r"\|\s*\w*[^|]*\|\s*[{(]").unwrap()
        .find_iter(source)
        .count();

    // Count && and || operators
    count += Regex::new(r"(&&|\|\|)").unwrap()
        .find_iter(source)
        .count();

    Ok(count + 1) // baseline complexity = 1
}

pub fn compute_all(
    files: &[crate::models::ParsedFile],
) -> Vec<(String, String, usize)> {
    let mut results = Vec::new();

    for pf in files {
        for func in &pf.functions {
            let score = compute_complexity(func).unwrap_or(1);
            results.push((pf.path.clone(), func.name.clone(), score));
        }
    }

    results
}
