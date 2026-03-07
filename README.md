# 🌌 Gravity: Code Intelligence Dashboard

Gravity is a NEXT-GEN code analysis platform designed to give developers deep insights into their codebases. It provides a visual and data-driven overview of project structure, complexity, and dependencies.

## ✨ Key Features

- **📊 Comprehensive Summary**: Instant overview of file counts, function totals, and average complexity across your entire project.
- **🗺️ Interactive Dependency Graph**: Visualize how your modules and files relate to each other with a dynamic, graph-based view.
- **⚙️ Complexity Scoring**: Identify "hotspots" in your code using advanced complexity analysis based on function structure and logic.
- **📁 File Explorer**: Detailed breakdown of line counts and metrics for every file in your repository.
- **🚀 Ultra-Fast Analysis**: Powered by Rust and Tree-sitter for lightning-fast parsing and processing.

## 🦀 Current Language Support

**Currently, Gravity exclusively supports Rust analysis.** 

It is designed to parse `.rs` files to extract functions, structs, imports, and calculate cyclomatic complexity. Support for other languages (Python, Go, JavaScript/TypeScript, etc.) is a top priority for our roadmap and we welcome contributions in this area!

## 🛠️ Technology Stack

Gravity is built with a high-performance, type-safe stack:

- **Frontend**: [Leptos](https://leptos.dev/) (Rust WASM)
- **Backend**: [Axum](https://github.com/tokio-rs/axum) (Rust)
- **Database**: [PostgreSQL](https://www.postgresql.org/)
- **Parsing**: [Tree-sitter](https://tree-sitter.github.io/tree-sitter/)
- **Infrastructure**: [Docker](https://www.docker.com/) & [Nginx](https://www.nginx.com/)

## 🚀 Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

### Installation & Deployment

1. **Clone the repository**:
   ```bash
   git clone https://github.com/AyushKatre05/Gravity.git
   cd Gravity
   ```

2. **Start the environment**:
   ```bash
   docker-compose up -d
   ```

3. **Access the Dashboard**:
   Open your browser and navigate to `http://localhost`.

### ☁️ Deploy to Render

Choose one of the following methods to deploy to Render:

#### Option 1: Render Blueprint (Easiest, may require card)
1. **Fork this repository**.
2. Go to **New +** > **Blueprint**.
3. Select this repo and click **Apply**.

#### Option 2: Manual Setup (Render Free Tier)
If you are on the free tier, follow these steps exactly:

1.  **Step 1: Create a PostgreSQL Database**
    -   Go to **New +** > **PostgreSQL**.
    -   Name: `gravity-db` | Plan: **Free**.
    -   After creation, find the **Internal Database URL** on the DB's dashboard. Copy it.

2.  **Step 2: Create the Backend Service**
    -   Go to **New +** > **Web Service** > Connect this repo.
    -   Name: `gravity-backend` | Runtime: **Docker**.
    -   **Advanced** > **Docker Build Path**: `backend/Dockerfile`.
    -   **Advanced** > **Docker Context**: `backend` (⚠️ THIS IS THE FIX).
    -   **Environment Variables**:
        -   `DATABASE_URL`: (Paste the URL from Step 1).
        -   `PORT`: `8080`.
    -   **Wait for it to deploy.** Once green, copy the **Internal Hostname** found in the "Settings" or main dashboard (looks like `http://gravity-backend:8080`).

3.  **Step 3: Create the Gateway/Frontend Service**
    -   Go to **New +** > **Web Service** > Connect this repo.
    -   Name: `gravity-gateway` | Runtime: **Docker**.
    -   **Advanced** > **Docker Build Path**: `nginx/Dockerfile`.
    -   **Advanced** > **Docker Context**: `.` (The root folder).
    -   **Environment Variables**:
        -   `BACKEND_URL`: (Paste the Internal Hostname from Step 2, e.g., `http://gravity-backend:8080`).

4.  **Done!**
    -   Your project is now live at the **Onrender URL** of the `gravity-gateway` service.

## 🤝 Contributing

We are looking for help to make Gravity multi-lingual! If you are interested in adding support for your favorite programming language, please check out our [CONTRIBUTING.md](file:///c:/Users/ayush/OneDrive/Desktop/project/Gravity/CONTRIBUTING.md) guide.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
