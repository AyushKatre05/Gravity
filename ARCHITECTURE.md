# 🏗 Architecture Guide

This document explains how Gravity is structured and how components communicate.

## System Overview

```
┌──────────────────────────────────────────────────────────────┐
│                     User's Browser                           │
│  Leptos WASM App (Compiled Rust to WebAssembly)             │
│                                                              │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │ • Dashboard UI (Summary, Files, Graph, Complexity)      │ │
│  │ • Event Handlers (Click, Input)                         │ │
│  │ • State Management (Signals, Resources)                 │ │
│  └──────────────────────────────────────────────────────────┘│
└────────────────────────────┬─────────────────────────────────┘
                             │
                    HTTP API (JSON)
                             │
┌────────────────────────────▼─────────────────────────────────┐
│                    Nginx (Reverse Proxy)                      │
│         Routes requests to static files or backend            │
│                                                               │
│  • Port 80 (HTTP)                                             │
│  • Serves static files (JS, WASM, CSS)                        │
│  • Proxies /api/* to backend                                  │
└────────────────────────────┬─────────────────────────────────┘
                             │
            ┌────────────────┴─────────────────┐
         /api/*                          Static files
            │                                   │
┌───────────▼─────────────────┐     ┌─────────▼────────────┐
│  Axum Backend (Port 8080)   │     │ Nginx Static Files   │
│  Rust HTTP Server           │     │                      │
│                             │     │ • index.html         │
│ ┌────────────────────────┐  │     │ • app.js (WASM)      │
│ │ API Routes             │  │     │ • styles.css         │
│ │ • /analyze    (POST)   │  │     │ • images/icons       │
│ │ • /summary    (GET)    │  │     └──────────────────────┘
│ │ • /files      (GET)    │  │
│ │ • /graph      (GET)    │  │
│ │ • /complexity (GET)    │  │
│ │ • /health     (GET)    │  │
│ └──────────┬─────────────┘  │
│            │                │
│ ┌──────────▼─────────────┐  │
│ │ Core Services          │  │
│ │ • parser (Tree-sitter) │  │
│ │ • graph (Petgraph)     │  │
│ │ • complexity (Scoring) │  │
│ │ • db (SQLx)            │  │
│ └──────────┬─────────────┘  │
└────────────┬─────────────────┘
             │
┌────────────▼──────────────────────┐
│  PostgreSQL Database               │
│  Stores analysis results           │
│                                    │
│  Tables:                           │
│  • projects                        │
│  • files                           │
│  • functions                       │
│  • imports/dependencies            │
│  • complexity_scores               │
└────────────────────────────────────┘
```

---

## Component Details

### Frontend (Leptos WASM)

**Location**: `frontend/src/`

**Files**:
- `main.rs` - Entry point, mounts app to DOM
- `app.rs` - Main component and all UI components

**How it works**:
1. User opens app
2. Leptos mounts `<App>` component
3. App renders UI and listens for user actions
4. On "Analyze" click, sends HTTP POST to `/api/analyze`
5. Receives project_id back
6. Fetches data from other endpoints for display

**Key Components**:
- `App` - Root component, manages current tab, project state
- `SummaryPanel` - Shows quick stats
- `FilesPanel` - Table of analyzed files
- `GraphPanel` - Visual dependency graph
- `ComplexityPanel` - Function complexity scores
- `StatCard` - Reusable stat display
- `LoadingCard` - Shows while loading
- `EmptyState` - Shows when no data

**Technology Stack**:
- Leptos 0.6 - UI framework
- Gloo-net - HTTP client
- WebAssembly - Runs in browser

---

### Backend (Axum)

**Location**: `backend/src/`

**Files**:
- `main.rs` - Server startup, initialization
- `api.rs` - API route handlers
- `db.rs` - Database operations
- `parser.rs` - Code parsing (tree-sitter)
- `graph.rs` - Dependency graph building
- `complexity.rs` - Complexity scoring
- `models.rs` - Data structures

**How it works**:

1. **Server Startup**:
   - Load environment variables
   - Connect to database
   - Run migrations
   - Start HTTP server on port 8080

2. **Analyze Endpoint** (`POST /api/analyze`):
   ```
   1. Receive request with GitHub URL or path
   2. If GitHub URL: Clone repo to temp directory
   3. Parse all .rs files with tree-sitter
   4. Extract:
      - Functions and their signatures
      - Imports and dependencies
      - Module structure
   5. Calculate complexity for each function
   6. Build dependency graph
   7. Insert into database
   8. Return project_id and basic stats
   9. Cleanup temp directory
   ```

3. **Summary Endpoint** (`GET /api/summary?project_id=...`):
   ```
   1. Query database for project
   2. Count files, functions, imports
   3. Calculate average complexity
   4. Identify dead code candidates
   5. Generate architecture notes
   6. Return JSON response
   ```

4. **Other Endpoints**:
   - `/files` - Get list of files with line counts
   - `/graph` - Get nodes and edges for dependency graph
   - `/complexity` - Get functions sorted by complexity score
   - `/health` - Simple health check for docker

**Technology Stack**:
- Axum - Web framework
- Tokio - Async runtime
- SQLx - Type-safe database queries
- Tree-sitter - Code parsing
- Petgraph - Graph algorithms

---

### Database (PostgreSQL)

**Location**: `backend/migrations/`

**Schema**:

```sql
-- Projects
CREATE TABLE projects (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  created_at TIMESTAMP,
);

-- Files analyzed
CREATE TABLE files (
  id UUID PRIMARY KEY,
  project_id UUID REFERENCES projects,
  path TEXT,
  line_count INTEGER,
);

-- Functions found
CREATE TABLE functions (
  id UUID PRIMARY KEY,
  file_id UUID REFERENCES files,
  name TEXT,
  line_start INTEGER,
  line_end INTEGER,
);

-- Imports/dependencies
CREATE TABLE imports (
  id UUID PRIMARY KEY,
  file_id UUID REFERENCES files,
  module_name TEXT,
);

-- Complexity scores
CREATE TABLE complexity_scores (
  id UUID PRIMARY KEY,
  function_id UUID REFERENCES functions,
  score INTEGER,
);
```

**Migrations**:
- `001_create_projects.sql` - Projects table
- `002_create_files.sql` - Files table
- `003_create_functions.sql` - Functions table
- `004_create_dependencies.sql` - Dependencies table
- `005_create_complexities.sql` - Complexity table

---

## Data Flow: Complete Example

### User analyzes `https://github.com/tokio-rs/tokio`

1. **Frontend**:
   - User enters URL and clicks "Analyze"
   - Sends: `POST /api/analyze { github_url: "https://github.com/tokio-rs/tokio" }`

2. **Backend - Analyze Handler**:
   - Clones repo to `/tmp/gravity-<uuid>`
   - Calls `parser::parse_directory()`

3. **Parser Module**:
   - Walks directory recursively
   - For each `.rs` file:
     - Parse with tree-sitter
     - Extract functions, structs, traits
     - Record line numbers
     - Collect import statements

4. **Complexity Module**:
   - For each function:
     - Count cyclomatic complexity
     - Store score (1-20+)

5. **Database**:
   - Insert project record
   - Insert files, functions, imports
   - Insert complexity scores

6. **Backend Response**:
   - Returns `{ project_id: "abc-123", files_analyzed: 145, ... }`

7. **Frontend**:
   - Stores project_id in signal
   - Fetches `/api/summary?project_id=abc-123`
   - Renders SummaryPanel with data

8. **User sees**:
   - 📊 145 files analyzed
   - ⚙️ 2,345 functions found
   - 🌡 Avg complexity 3.4

---

## Performance Considerations

### Frontend
- **Bundle Size**: ~500KB (WASM + JS)
- **Lazy Loading**: Resources load data on-demand
- **Caching**: HTTP cache headers set by Nginx

### Backend
- **Request Parsing**: Sequential file parsing could bottleneck
- **Database**: Indexed queries for fast lookups
- **Memory**: Large repos kept in temp directory only during analysis

### Database
- **Indexes**: On `project_id`, `file_id` for fast joins
- **Query**: Summary aggregation could be cached

---

## Extension Points

Want to add features? Here are good places:

1. **Add new analysis type**:
   - Create module in `backend/src/new_analysis.rs`
   - Add function to compute results
   - Add API endpoint in `api.rs`
   - Add table in database migration
   - Create frontend panel component

2. **Add new UI component**:
   - Create in `frontend/src/app.rs`
   - Add to Tab enum
   - Call API endpoint for data
   - Render results

3. **Improve parser accuracy**:
   - Enhance `backend/src/parser.rs`
   - Use tree-sitter queries
   - Extract more code metadata

4. **Add GitHub integration**:
   - Support private repos with token
   - Webhooks for auto-analysis
   - PR comments with results

---

## Key Design Decisions

| Decision | Reason |
|----------|--------|
| Rust for frontend | Type safety, performance, small bundle |
| Rust for backend | Performance, thread safety, ecosystem |
| PostgreSQL | Proven, reliable, good Rust support |
| Docker | Consistent env, easy deployment |
| Nginx reverse proxy | Decouples frontend server, HTTP best practices |
| Tree-sitter for parsing | Accurate, incremental, maintained |

---

## Common Patterns

### Creating an API Endpoint

```rust
// In api.rs
async fn my_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<MyParams>,
) -> Result<Json<MyResponse>, (StatusCode, String)> {
    // Logic here
    Ok(Json(response))
}

// Add to router
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/my-endpoint", get(my_handler))
        // ...
}
```

### Creating a Frontend Component

```rust
#[component]
fn MyComponent(project_id: ReadSignal<Option<String>>) -> impl IntoView {
    let data = create_resource(project_id, |pid| async move {
        // Fetch data
    });

    view! {
        <Suspense fallback=|| view! { <LoadingCard /> }>
            {move || data.get().map(|d| view! { /* render */ })}
        </Suspense>
    }
}
```

---

## Debugging

### Backend Logs
```bash
RUST_LOG=debug docker-compose logs backend
```

### Database Queries
```bash
docker-compose exec postgres psql -U user -d gravity
SELECT * FROM projects;
```

### Frontend Console
```javascript
// Browser console
console.log('Current state:', state);
```

---

## References

- [Axum Docs](https://docs.rs/axum)
- [Leptos Guide](https://leptos.dev/)
- [Tree-sitter Docs](https://tree-sitter.github.io/)
- [SQLx Docs](https://sqlx.rs/)
