use leptos::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeRequest {
    pub project_name: Option<String>,
    pub path: Option<String>,
    pub github_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeResponse {
    pub project_id: String,
    pub files_analyzed: usize,
    pub functions_found: usize,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSummary {
    pub project_id: String,
    pub project_name: String,
    pub total_files: i64,
    pub total_functions: i64,
    pub total_structs: i64,
    pub total_imports: i64,
    pub avg_complexity: f64,
    pub dead_code_candidates: Vec<String>,
    pub architecture_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub id: String,
    pub path: String,
    pub module_name: Option<String>,
    pub line_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityItem {
    pub function_name: String,
    pub file_path: String,
    pub score: i32,
    pub line_start: i32,
    pub line_end: i32,
}

// ───  ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Tab {
    Summary,
    Files,
    Graph,
    Complexity,
}
