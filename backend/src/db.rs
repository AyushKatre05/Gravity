use anyhow::{Context, Result};
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

use crate::models::{
    AnalysisSummary, ComplexityItem, Dependency, FileEntry,
    FunctionEntry, GraphData, GraphEdge, GraphNode, ParsedFile,
    ParsedFunction, Project,
};
pub async fn init_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .context("Failed to connect to Postgres")?;

    Ok(pool)
}
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("Failed to run migrations")?;

    Ok(())
}

pub async fn upsert_project(pool: &PgPool, name: &str, path: &str) -> Result<Project> {
    let existing = sqlx::query_as!(
        Project,
        r#"SELECT id, name, path, created_at, updated_at
           FROM projects WHERE name = $1 AND path = $2
           LIMIT 1"#,
        name,
        path
    )
    .fetch_optional(pool)
    .await?;

    if let Some(p) = existing {
        sqlx::query!(
            "UPDATE projects SET updated_at = NOW() WHERE id = $1",
            p.id
        )
        .execute(pool)
        .await?;
        return Ok(p);
    }

    let project = sqlx::query_as!(
        Project,
        r#"INSERT INTO projects (id, name, path, created_at, updated_at)
           VALUES ($1, $2, $3, NOW(), NOW())
           RETURNING id, name, path, created_at, updated_at"#,
        Uuid::new_v4(),
        name,
        path
    )
    .fetch_one(pool)
    .await
    .context("Failed to insert project")?;

    Ok(project)
}
