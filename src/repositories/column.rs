use sqlx::{postgres::PgRow, PgPool, Row};

use crate::errors::AppResult;

#[derive(Debug, Clone)]
pub struct ColumnRecord {
    pub id: i64,
    pub title: String,
    pub image_url: String,
    pub content: String,
    pub published_at: String,
}

pub async fn fetch_total_columns(pool: &PgPool) -> AppResult<i64> {
    let row = sqlx::query(
        r#"
        SELECT COUNT(*)::BIGINT AS total
        FROM columns
        WHERE deleted_at IS NULL
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.get("total"))
}

pub async fn fetch_columns(pool: &PgPool, limit: i64, offset: i64) -> AppResult<Vec<ColumnRecord>> {
    let rows = sqlx::query(
        r#"
        SELECT
            id,
            title,
            COALESCE(image_url, '') AS image_url,
            content,
            TO_CHAR(created_at, 'YYYY.MM.DD HH24:MI') AS published_at
        FROM columns
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(map_column_row).collect())
}

pub async fn fetch_column_by_id(pool: &PgPool, column_id: i64) -> AppResult<Option<ColumnRecord>> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            title,
            COALESCE(image_url, '') AS image_url,
            content,
            TO_CHAR(created_at, 'YYYY.MM.DD HH24:MI') AS published_at
        FROM columns
        WHERE id = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(column_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(map_column_row))
}

fn map_column_row(row: PgRow) -> ColumnRecord {
    ColumnRecord {
        id: row.get("id"),
        title: row.get("title"),
        image_url: row.get("image_url"),
        content: row.get("content"),
        published_at: row.get("published_at"),
    }
}