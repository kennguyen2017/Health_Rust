use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct ColumnListQuery {
    #[param(example = 8)]
    pub limit: Option<i64>,
    #[param(example = 0)]
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ColumnListResponse {
    pub tabs: Vec<ColumnTab>,
    pub articles: Vec<ColumnArticle>,
    pub pagination: ColumnPagination,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateColumnRequest {
    #[schema(example = 3)]
    pub user_id: i64,
    #[schema(example = "How to keep a health journal consistent")]
    pub title: String,
    #[schema(example = "A practical system for writing short, useful health notes every day.")]
    pub content: String,
    #[schema(example = "https://images.unsplash.com/photo-1490645935967-10de6ba17061")]
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateColumnResponse {
    pub article: ColumnDetailArticle,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDetailResponse {
    pub article: ColumnDetailArticle,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct ColumnTab {
    #[schema(example = "column")]
    pub id: &'static str,
    #[schema(example = "RECOMMENDED COLUMN")]
    pub title: &'static str,
    #[schema(example = "オススメ")]
    pub subtitle: &'static str,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct ColumnArticle {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = "How to build a sustainable morning routine")]
    pub title: String,
    #[schema(example = "2026.04.12 02:00")]
    pub date: String,
    #[schema(example = "https://images.unsplash.com/photo-1500530855697-b586d89ba3ee")]
    pub image: String,
    #[schema(example = json!(["#健康", "#朝習慣"]))]
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDetailArticle {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = "How to build a sustainable morning routine")]
    pub title: String,
    #[schema(example = "2026.04.12 02:00")]
    pub date: String,
    #[schema(example = "https://images.unsplash.com/photo-1500530855697-b586d89ba3ee")]
    pub image: String,
    #[schema(example = json!(["#健康", "#朝習慣"]))]
    pub tags: Vec<String>,
    #[schema(example = "A strong morning routine starts with consistency, not intensity.")]
    pub content: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ColumnPagination {
    #[schema(example = 8)]
    pub limit: i64,
    #[schema(example = 0)]
    pub offset: i64,
    #[schema(example = 16)]
    pub total: i64,
    #[schema(example = true)]
    pub has_more: bool,
}