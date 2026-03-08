use leptos::{
    component, create_signal, view, IntoView,
    spawn_local, ReadSignal,
    CollectView, SignalGet, SignalSet, Suspense, create_resource,
};
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};

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
        set_analyzing.set(true);
        set_error.set(None);
        let github_url_val = github_url.get();
        spawn_local(async move {
            let req = Request::post("/api/analyze")
                .header("Content-Type", "application/json")
                .json(&AnalyzeRequest {
                    project_name: Some("gravity-project".into()),
                    path: None,
                    github_url: if github_url_val.is_empty() { None } else { Some(github_url_val) },
                });
            
            match req {
                Ok(builder) => {
                    match builder.send().await {
                        Ok(resp) => {
                            if resp.ok() {
                                match resp.json::<AnalyzeResponse>().await {
                                    Ok(data) => {
                                        set_project_id.set(Some(data.project_id.clone()));
                                        set_analyze_msg.set(Some(data.message));
                                    }
                                    Err(e) => set_error.set(Some(format!("Parse error: {e}"))),
                                }
                            } else {
                                set_error.set(Some(format!("HTTP {}", resp.status())));
                            }
                        }
                        Err(e) => set_error.set(Some(format!("Request failed: {e}"))),
                    }
                }
                Err(e) => set_error.set(Some(format!("Request building failed: {e}"))),
            }
            set_analyzing.set(false);
        });
    };

    view! {
        <div class="min-h-screen flex flex-col bg-transparent">

            <header class="glass sticky" style="top: 0; z-index: 50;">
                <div class="max-w-7xl mx-auto px-4 md:px-6 py-4">
                    <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
                        <div class="flex items-center gap-3">
                            <div class="w-10 h-10 rounded-lg flex items-center justify-center"
                                 style="background: linear-gradient(135deg, #8b5cf6, #6366f1); flex-shrink: 0; box-shadow: 0 4px 12px rgba(139,92,246,0.3);">
                                <span class="text-xl font-bold font-heading text-white">"G"</span>
                            </div>
                            <div>
                                <h1 class="text-2xl font-bold font-heading text-gradient">"Gravity"</h1>
                                <p class="text-xs font-medium tracking-wide" style="color: var(--text-muted); opacity: 0.8;">"Code Intelligence Dashboard"</p>
                            </div>
                        </div>

                        <div class="flex flex-col md:flex-row items-stretch md:items-center gap-2 w-full md:w-auto">
                            <div class="relative flex-1 md:flex-none">
                                <input
                                    type="text"
                                    placeholder="https://github.com/owner/repo"
                                    on:input=move |ev| {
                                        if let Some(target) = ev.target() {
                                            if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                                                set_github_url.set(input.value());
                                            }
                                        }
                                    }
                                    prop:value=github_url
                                    class="w-full md:w-64 px-4 py-2.5 rounded-lg text-sm transition-all outline-none focus:ring-2 focus:ring-purple-500/50"
                                    style="background: rgba(15, 23, 42, 0.6); border: 1px solid var(--border); color: var(--text-primary); backdrop-filter: blur(8px);"
                                />
                            </div>
                            <button
                                on:click=run_analyze
                                disabled=analyzing
                                class="px-4 md:px-6 py-2.5 rounded-lg text-sm font-semibold transition-all whitespace-nowrap"
                                style="background: linear-gradient(135deg, #8b5cf6, #6366f1); color: white;"
                            >
                                {move || if analyzing.get() { "Analyzing..." } else { "Analyze" }}
                            </button>
                        </div>
                    </div>

                    <div class="flex flex-col md:flex-row items-start md:items-center gap-2 mt-4 md:mt-0">
                        {move || error.get().map(|e| view! {
                            <div class="w-full md:w-auto px-3 py-2 rounded-lg text-sm animate-fadeIn"
                                 style="background: rgba(239,68,68,0.15); color: var(--danger);">
                                "[ERR] " {e}
                            </div>
                        })}
                        {move || analyze_msg.get().map(|m| view! {
                            <div class="w-full md:w-auto px-3 py-2 rounded-lg text-sm animate-fadeIn"
                                 style="background: var(--bg-secondary); border: 1px solid var(--border); color: var(--text-primary);">
                                "[OK] " {m}
                            </div>
                        })}
                    </div>
                </div>
            </header>

            <nav class="sticky top-14 md:relative max-w-7xl mx-auto px-4 md:px-6 pt-4 md:pt-8 z-40">
                <div class="glass flex gap-2 p-1.5 rounded-xl w-full overflow-x-auto shadow-lg">
                    {[
                        (Tab::Summary,    "Summary"),
                        (Tab::Files,      "Files"),
                        (Tab::Graph,      "Graph"),
                        (Tab::Complexity, "Complexity"),
                    ].into_iter().map(|(tab, label)| {
                        let tab_clone = tab.clone();
                        view! {
                            <button
                                on:click=move |_| set_tab.set(tab_clone.clone())
                                class="px-4 py-2 rounded-lg text-sm font-semibold transition-all whitespace-nowrap hover:bg-white/5"
                                style=move || {
                                    if active_tab.get() == tab.clone() {
                                        "background: rgba(139, 92, 246, 0.2); color: white; border: 1px solid rgba(139, 92, 246, 0.4);"
                                    } else {
                                        "color: var(--text-muted); background: transparent; border: 1px solid transparent;"
                                    }
                                }
                            >{label}</button>
                        }
                    }).collect_view()}
                </div>
            </nav>

            <main class="flex-1 max-w-7xl mx-auto px-4 md:px-6 py-6 w-full">
                {move || match active_tab.get() {
                    Tab::Summary    => view! { <SummaryPanel project_id=project_id /> }.into_view(),
                    Tab::Files      => view! { <FilesPanel project_id=project_id /> }.into_view(),
                    Tab::Graph      => view! { <GraphPanel project_id=project_id /> }.into_view(),
                    Tab::Complexity => view! { <ComplexityPanel project_id=project_id /> }.into_view(),
                }}
            </main>

            <footer class="glass mt-auto border-t" style="border-top-color: var(--border);">
                <div class="max-w-7xl mx-auto px-4 md:px-6 py-6 text-center text-sm font-medium" style="color: var(--text-muted);">
                    <span class="opacity-75">"Gravity • Code Intelligence Dashboard • "</span>
                    <a href="https://github.com/AyushKatre05/Gravity" class="transition-colors hover:text-white" style="color: var(--accent-light);">{"Open Source"}</a>
                </div>
            </footer>
        </div>
    }
}

#[component]
fn SummaryPanel(project_id: ReadSignal<Option<String>>) -> impl IntoView {
    let summary = create_resource(
        move || project_id.get(),
        |pid| async move {
        let url = match &pid {
            Some(id) => format!("/api/summary?project_id={id}"),
            None => "/api/summary".into(),
        };
        Request::get(&url).send().await.ok()?
            .json::<AnalysisSummary>().await.ok()
    });

    view! {
        <div>
            <Suspense fallback=move || view! { <LoadingCard /> }>
                {move || summary.get().flatten().map(|s| view! {
                    <div>
                        <div class="mb-8 pl-1">
                            <h2 class="text-3xl font-bold font-heading text-white">
                                {s.project_name.clone()}
                            </h2>
                            <p class="text-sm mt-2 font-medium" style="color: var(--text-muted);">"Project analysis results"</p>
                        </div>
                        <div class="grid grid-cols-2 gap-4 mb-8 lg:grid-cols-4">
                            <StatCard label="Files" value=s.total_files.to_string() icon="📄" />
                            <StatCard label="Functions" value=s.total_functions.to_string() icon="⚡" />
                            <StatCard label="Imports" value=s.total_imports.to_string() icon="📦" />
                            <StatCard label="Avg Complexity" 
                                      value=format!("{:.1}", s.avg_complexity) icon="🧠" />
                        </div>
                        <div class="grid gap-6 lg:grid-cols-2">
                            <div class="glass-card p-6 rounded-2xl">
                                <div class="flex items-center gap-2 mb-4">
                                    <span class="text-xl">"🏛️"</span>
                                    <h3 class="font-semibold font-heading text-lg" style="color: var(--accent-light);">"Architecture Notes"</h3>
                                </div>
                                <ul class="space-y-3">
                                    {s.architecture_notes.iter().map(|n| view! {
                                        <li class="text-sm flex gap-3 items-start animate-fadeIn">
                                            <span class="mt-1.5 w-1.5 h-1.5 rounded-full flex-shrink-0" style="background: var(--accent);"></span>
                                            <span style="color: var(--text-primary); opacity: 0.9;">{n.clone()}</span>
                                        </li>
                                    }).collect_view()}
                                </ul>
                            </div>
                            <div class="glass-card p-6 rounded-2xl">
                                <div class="flex items-center gap-2 mb-4">
                                    <span class="text-xl">"⚠️"</span>
                                    <h3 class="font-semibold font-heading text-lg" style="color: var(--warning);">"Dead Code Candidates"</h3>
                                </div>
                                {if s.dead_code_candidates.is_empty() {
                                    view! { <p class="text-sm" style="color: var(--success);">[OK] No dead code detected.</p> }.into_view()
                                } else {
                                    view! {
                                        <ul class="space-y-1">
                                            {s.dead_code_candidates.iter().map(|fn_name| view! {
                                                <li class="text-sm mono px-2 py-1 rounded"
                                                    style="background: rgba(210,153,34,0.1); color: var(--warning);">
                                                    {fn_name.clone()}
                                                </li>
                                            }).collect_view()}
                                        </ul>
                                    }.into_view()
                                }}
                            </div>
                        </div>
                    </div>
                })}
                {move || {
                    if summary.get().is_none() || summary.get().flatten().is_none() {
                        view! {
                            <EmptyState
                                icon="[SUM]"
                                title="No analysis yet"
                                hint="Click button to analyze the mounted project."
                            />
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn FilesPanel(project_id: ReadSignal<Option<String>>) -> impl IntoView {
    let files = create_resource(
        move || project_id.get(),
        |pid| async move {
        let url = match &pid {
            Some(id) => format!("/api/files?project_id={id}"),
            None => "/api/files".into(),
        };
        Request::get(&url).send().await.ok()?
            .json::<Vec<FileEntry>>().await.ok()
    });

    view! {
        <Suspense fallback=move || view! { <LoadingCard /> }>
            {move || files.get().flatten().map(|fs| {
                if fs.is_empty() {
                    view! { <EmptyState icon="[FILE]" title="No files found" hint="Run analysis first." /> }.into_view()
                } else {
                    view! {
                        <div class="glass-card rounded-2xl overflow-hidden shadow-xl animate-fadeIn">
                            <table class="w-full text-sm">
                                <thead>
                                    <tr class="glass">
                                        <th class="text-left px-6 py-4 font-semibold font-heading" style="color: var(--text-muted);">"File Path"</th>
                                        <th class="text-left px-6 py-4 font-semibold font-heading" style="color: var(--text-muted);">"Module"</th>
                                        <th class="text-right px-6 py-4 font-semibold font-heading" style="color: var(--text-muted);">"Lines"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {fs.iter().enumerate().map(|(i, f)| {
                                        let bg = if i % 2 == 0 { "rgba(30, 41, 59, 0.3)" } else { "rgba(15, 23, 42, 0.3)" };
                                        let row_style = format!("background: {};", bg);
                                        view! {
                                            <tr style=row_style class="transition-colors hover:bg-white/5">
                                                <td class="px-6 py-3 mono" style="color: var(--accent-light); font-size: 0.85rem;">
                                                    {f.path.clone()}
                                                </td>
                                                <td class="px-6 py-3 font-medium" style="color: var(--text-muted); opacity: 0.9;">
                                                    {f.module_name.clone().unwrap_or_default()}
                                                </td>
                                                <td class="px-6 py-3 text-right mono font-bold" style="color: var(--text-primary);">
                                                    {f.line_count}
                                                </td>
                                            </tr>
                                        }
                                    }).collect_view()}
                                </tbody>
                            </table>
                        </div>
                    }.into_view()
                }
            })}
        </Suspense>
    }
}

#[component]
fn GraphPanel(project_id: ReadSignal<Option<String>>) -> impl IntoView {
    let graph = create_resource(
        move || project_id.get(),
        |pid| async move {
        let url = match &pid {
            Some(id) => format!("/api/graph?project_id={id}"),
            None => "/api/graph".into(),
        };
        Request::get(&url).send().await.ok()?
            .json::<GraphData>().await.ok()
    });

    view! {
        <Suspense fallback=move || view! { <LoadingCard /> }>
            {move || graph.get().flatten().map(|g| {
                if g.nodes.is_empty() {
                    return view! { <EmptyState icon="[NET]" title="No graph data" hint="Run analysis first." /> }.into_view();
                }

                let nodes_json = serde_json::to_string(&g.nodes).unwrap_or_default();
                let edges_json = serde_json::to_string(&g.edges).unwrap_or_default();

                let script_content = format!(r#"
                    setTimeout(function() {{
                        var rawNodes = {nodes_json};
                        var rawEdges = {edges_json};
                        var nodes = new vis.DataSet(rawNodes.map(function(n) {{
                            var color = n.kind === 'file' ? '#8b5cf6' : n.kind === 'module' ? '#6366f1' : '#475569';
                            return {{ id: n.id, label: n.label, color: {{ background: color, border: '#c4b5fd' }},
                                     font: {{ color: '#f8fafc', size: 14 }}, shape: 'box',
                                     borderWidth: 1.5, shadow: {{ enabled: true, color: 'rgba(0,0,0,0.4)', size: 10 }} }};
                        }}));
                        var edges = new vis.DataSet(rawEdges.map(function(e) {{
                            return {{ from: e.from, to: e.to, arrows: 'to',
                                     color: {{ color: '#64748b', highlight: '#a855f7' }},
                                     smooth: {{ type: 'cubicBezier' }} }};
                        }}));
                        var container = document.getElementById('graph-container');
                        if (container) {{
                            new vis.Network(container, {{ nodes: nodes, edges: edges }}, {{
                                layout: {{ improvedLayout: true }},
                                physics: {{ barnesHut: {{ gravitationalConstant: -3000, springLength: 150 }} }},
                                interaction: {{ hover: true, tooltipDelay: 100, zoomSpeed: 0.8 }}
                            }});
                        }}
                    }}, 50);
                "#);
                
                let _ = js_sys::eval(&script_content);

                view! {
                    <div class="animate-fadeIn">
                        <div class="mb-5 flex items-center gap-4">
                            <span class="text-sm px-4 py-1.5 rounded-full font-medium shadow-sm transition-all"
                                  style="background: rgba(139, 92, 246, 0.2); border: 1px solid rgba(139, 92, 246, 0.3); color: var(--accent-light);">
                                {format!("{} nodes", g.nodes.len())}
                            </span>
                            <span class="text-sm px-4 py-1.5 rounded-full font-medium shadow-sm transition-all"
                                  style="background: rgba(99, 102, 241, 0.15); border: 1px solid rgba(99, 102, 241, 0.2); color: var(--primary-light);">
                                {format!("{} edges", g.edges.len())}
                            </span>
                        </div>
                        <div id="graph-container" class="shadow-2xl transition-all duration-300 hover:shadow-purple-500/10"></div>
                    </div>
                }.into_view()
            })}
        </Suspense>
    }
}

#[component]
fn ComplexityPanel(project_id: ReadSignal<Option<String>>) -> impl IntoView {
    let items = create_resource(
        move || project_id.get(),
        |pid| async move {
        let url = match &pid {
            Some(id) => format!("/api/complexity?project_id={id}"),
            None => "/api/complexity".into(),
        };
        Request::get(&url).send().await.ok()?
            .json::<Vec<ComplexityItem>>().await.ok()
    });

    view! {
        <Suspense fallback=move || view! { <LoadingCard /> }>
            {move || items.get().flatten().map(|cx| {
                if cx.is_empty() {
                    return view! { <EmptyState icon="[COMP]" title="No complexity data" hint="Run analysis first." /> }.into_view();
                }
                view! {
                    <div class="rounded-xl overflow-hidden" style="border: 1px solid var(--border);">
                        <table class="w-full text-sm">
                            <thead>
                                <tr style="background: var(--bg-secondary);">
                                    <th class="text-left px-4 py-3 font-semibold" style="color: var(--text-muted);">"Function"</th>
                                    <th class="text-left px-4 py-3 font-semibold" style="color: var(--text-muted);">"File"</th>
                                    <th class="text-center px-4 py-3 font-semibold" style="color: var(--text-muted);">"Lines"</th>
                                    <th class="text-center px-4 py-3 font-semibold" style="color: var(--text-muted);">"Score"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {cx.iter().enumerate().map(|(i, item)| {
                                    let bg = if i % 2 == 0 { "var(--bg-card)" } else { "var(--bg-secondary)" };
                                    let score_color = if item.score >= 10 {
                                        "var(--danger)"
                                    } else if item.score >= 5 {
                                        "var(--warning)"
                                    } else {
                                        "var(--success)"
                                    };
                                    let score_badge_bg = if item.score >= 10 {
                                        "rgba(248,81,73,0.15)"
                                    } else if item.score >= 5 {
                                        "rgba(210,153,34,0.15)"
                                    } else {
                                        "rgba(63,185,80,0.15)"
                                    };
                                    let row_style = format!("background: {};", bg);
                                    let score_style = format!("background: {}; color: {};", score_badge_bg, score_color);
                                    let line_range = format!("{}-{}", item.line_start, item.line_end);

                                    view! {
                                        <tr style=row_style>
                                            <td class="px-4 py-2 mono font-medium"
                                                style="color: var(--accent-light); font-size: 0.8rem;">
                                                {item.function_name.clone()}
                                            </td>
                                            <td class="px-4 py-2 mono"
                                                style="color: var(--text-muted); font-size: 0.75rem;">
                                                {item.file_path.rsplit('/').next().unwrap_or("").to_string()}
                                            </td>
                                            <td class="px-4 py-2 text-center mono" style="color: var(--text-muted);">
                                                {line_range}
                                            </td>
                                            <td class="px-4 py-2 text-center">
                                                <span class="px-2 py-1 rounded-md text-xs font-bold mono"
                                                      style=score_style>
                                                    {item.score}
                                                </span>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()}
                            </tbody>
                        </table>
                    </div>
                }.into_view()
            })}
        </Suspense>
    }
}

#[component]
fn StatCard(
    label: &'static str,
    value: String,
    icon: &'static str,
    #[prop(optional)] subtitle: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="p-4 md:p-6 rounded-xl card-hover" style="background: var(--bg-card); border: 1px solid var(--border);">
            <div class="flex items-start gap-3 mb-3">
                <span class="text-2xl md:text-3xl">{icon}</span>
                <div>
                    <p class="text-xs font-semibold uppercase tracking-wider" style="color: var(--text-muted);">{label}</p>
                    {subtitle.map(|s| view! {
                        <p class="text-xs" style="color: var(--text-muted); margin-top: 2px;">{s}</p>
                    })}
                </div>
            </div>
            <div class="text-3xl md:text-4xl font-bold" style="color: var(--accent-light);">{value}</div>
        </div>
    }
}

#[component]
fn LoadingCard() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center py-24">
            <div class="flex flex-col items-center gap-4">
                <div class="w-10 h-10 rounded-full animate-spin"
                     style="border: 3px solid var(--border); border-top-color: var(--accent);"></div>
                <p class="text-sm" style="color: var(--text-muted);">"Loading data…"</p>
            </div>
        </div>
    }
}

#[component]
fn EmptyState(icon: &'static str, title: &'static str, hint: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-24 gap-4">
            <div class="text-5xl">{icon}</div>
            <h3 class="text-xl font-semibold" style="color: var(--text-primary);">{title}</h3>
            <p class="text-sm" style="color: var(--text-muted);">{hint}</p>
        </div>
    }
}
