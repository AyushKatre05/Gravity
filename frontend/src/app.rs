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

#[component]
pub fn App() -> impl IntoView {
    // State
    let (active_tab, set_tab)         = create_signal(Tab::Summary);
    let (project_id, set_project_id)  = create_signal::<Option<String>>(None);
    let (analyzing, set_analyzing)    = create_signal(false);
    let (error, set_error)            = create_signal::<Option<String>>(None);
    let (analyze_msg, set_analyze_msg)= create_signal::<Option<String>>(None);
    let (github_url, set_github_url)  = create_signal(String::new());

    let run_analyze = move |_| {
        set_analyzing(true);
        set_error(None);
        spawn_local(async move {
            match Request::post("/api/analyze")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&AnalyzeRequest {
                    project_name: Some("gravity-project".into()),
                    path: None,
                    github_url: if github_url().is_empty() { None } else { Some(github_url()) },
                }).unwrap())
                .send()
                .await
            {
                Ok(resp) => {
                    if resp.ok() {
                        match resp.json::<AnalyzeResponse>().await {
                            Ok(data) => {
                                set_project_id(Some(data.project_id.clone()));
                                set_analyze_msg(Some(data.message));
                            }
                            Err(e) => set_error(Some(format!("Parse error: {e}"))),
                        }
                    } else {
                        set_error(Some(format!("HTTP {}", resp.status())));
                    }
                }
                Err(e) => set_error(Some(format!("Request failed: {e}"))),
            }
            set_analyzing(false);
        });
    };

    view! {
        <div class="min-h-screen" style="background: var(--bg-primary);">

            <header style="background: var(--bg-secondary); border-bottom: 1px solid var(--border);">
                <div class="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div class="w-9 h-9 rounded-lg flex items-center justify-center"
                             style="background: linear-gradient(135deg, #7c3aed, #4f46e5);">
                            <span class="text-lg">⚡</span>
                        </div>
                        <div>
                            <h1 class="text-xl font-bold" style="color: var(--text-primary);">"Gravity"</h1>
                            <p class="text-xs" style="color: var(--text-muted);">"Code Intelligence Dashboard"</p>
                        </div>
                    </div>

                    <div class="flex items-center gap-3">
                        <input
                            type="text"
                            placeholder="https://github.com/owner/repo"
                            on:input=move |ev| set_github_url(event_value(&ev))
                            prop:value=github_url
                            class="px-3 py-2 rounded-lg text-sm w-64 transition-all"
                            style="background: var(--bg-card); border: 1px solid var(--border); color: var(--text-primary); outline: none;"
                        />
                        {move || error().map(|e| view! {
                            <span class="text-sm px-3 py-1 rounded-md"
                                  style="background: rgba(248,81,73,0.15); color: var(--danger);">{e}</span>
                        })}
                        {move || analyze_msg().map(|m| view! {
                            <span class="text-sm px-3 py-1 rounded-md"
                                  style="background: rgba(63,185,80,0.15); color: var(--success);">{m}</span>
                        })}
                        <button
                            on:click=run_analyze
                            disabled=analyzing
                            class="px-5 py-2 rounded-lg text-sm font-semibold transition-all"
                            style="background: linear-gradient(135deg, #7c3aed, #4f46e5); color: white; cursor: pointer;"
                        >
                            {move || if analyzing() { "Analyzing…" } else { "⚡ Run Analysis" }}
                        </button>
                    </div>
                </div>
            </header>

            <nav class="max-w-7xl mx-auto px-6 pt-6">
                <div class="flex gap-1 p-1 rounded-xl w-fit"
                     style="background: var(--bg-secondary); border: 1px solid var(--border);">
                    {[
                        (Tab::Summary,    "📊 Summary"),
                        (Tab::Files,      "📁 Files"),
                        (Tab::Graph,      "🔗 Graph"),
                        (Tab::Complexity, "🌡 Complexity"),
                    ].into_iter().map(|(tab, label)| {
                        let tab_clone = tab.clone();
                        view! {
                            <button
                                on:click=move |_| set_tab(tab_clone.clone())
                                class="px-4 py-2 rounded-lg text-sm font-medium transition-all"
                                style=move || {
                                    if active_tab() == tab.clone() {
                                        "background: var(--accent); color: white;"
                                    } else {
                                        "color: var(--text-muted); background: transparent;"
                                    }
                                }
                            >{label}</button>
                        }
                    }).collect_view()}
                </div>
            </nav>

            // ── Content ───────────────────────────────────────────────────
            <main class="max-w-7xl mx-auto px-6 py-6">
                {move || match active_tab() {
                    Tab::Summary    => view! { <SummaryPanel project_id=project_id /> }.into_view(),
                    Tab::Files      => view! { <FilesPanel project_id=project_id /> }.into_view(),
                    Tab::Graph      => view! { <GraphPanel project_id=project_id /> }.into_view(),
                    Tab::Complexity => view! { <ComplexityPanel project_id=project_id /> }.into_view(),
                }}
            </main>
        </div>
    }
}
