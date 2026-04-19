use sqlx::PgPool;

use crate::dto::column::{
    ColumnArticle, ColumnDetailArticle, ColumnDetailResponse, ColumnListQuery, ColumnListResponse,
    ColumnPagination, ColumnTab,
};
use crate::errors::{AppError, AppResult};
use crate::repositories;

const COLUMN_TABS: [ColumnTab; 4] = [
    ColumnTab {
        id: "column",
        title: "RECOMMENDED COLUMN",
        subtitle: "オススメ",
    },
    ColumnTab {
        id: "diet",
        title: "RECOMMENDED DIET",
        subtitle: "ダイエット",
    },
    ColumnTab {
        id: "beauty",
        title: "RECOMMENDED BEAUTY",
        subtitle: "美容",
    },
    ColumnTab {
        id: "health",
        title: "RECOMMENDED HEALTH",
        subtitle: "健康",
    },
];

pub async fn get_columns(pool: &PgPool, query: ColumnListQuery) -> AppResult<ColumnListResponse> {
    let limit = query.limit.unwrap_or(8).clamp(1, 24);
    let offset = query.offset.unwrap_or(0).max(0);
    let total = repositories::column::fetch_total_columns(pool).await?;
    let records = repositories::column::fetch_columns(pool, limit, offset).await?;
    let articles = records
        .into_iter()
        .map(|record| ColumnArticle {
            id: record.id,
            title: record.title.clone(),
            date: record.published_at,
            image: record.image_url,
            tags: infer_tags(&record.title, &record.content),
        })
        .collect::<Vec<_>>();

    Ok(ColumnListResponse {
        tabs: COLUMN_TABS.to_vec(),
        articles,
        pagination: ColumnPagination {
            limit,
            offset,
            total,
            has_more: offset + limit < total,
        },
    })
}

pub async fn get_column_detail(pool: &PgPool, column_id: i64) -> AppResult<ColumnDetailResponse> {
    let record = repositories::column::fetch_column_by_id(pool, column_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("column {column_id} not found")))?;

    Ok(ColumnDetailResponse {
        article: ColumnDetailArticle {
            id: record.id,
            title: record.title.clone(),
            date: record.published_at,
            image: record.image_url,
            tags: infer_tags(&record.title, &record.content),
            content: record.content,
        },
    })
}

fn infer_tags(title: &str, content: &str) -> Vec<String> {
    let haystack = format!("{} {}", title.to_lowercase(), content.to_lowercase());
    let mut tags = Vec::new();

    if haystack.contains("morning") || haystack.contains("breakfast") || haystack.contains("routine") {
        tags.push("#朝習慣".to_string());
    }

    if haystack.contains("nutrition") || haystack.contains("meal") || haystack.contains("protein") {
        tags.push("#栄養".to_string());
    }

    if haystack.contains("body") || haystack.contains("weight") || haystack.contains("tracking") {
        tags.push("#ボディメイク".to_string());
    }

    if haystack.contains("health") || haystack.contains("energy") || haystack.contains("water") {
        tags.push("#健康".to_string());
    }

    if tags.is_empty() {
        tags.push("#健康".to_string());
    }

    if tags.len() == 1 {
        tags.push("#コラム".to_string());
    }

    tags.truncate(2);
    tags
}