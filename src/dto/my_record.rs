use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::dto::top::ChartPoint;

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct MyRecordQuery {
    #[param(example = 3)]
    pub user_id: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MyRecordResponse {
    pub highlights: Vec<RecordHighlight>,
    #[schema(example = "2026.05.21")]
    pub chart_date: String,
    pub chart_filters: Vec<RecordChartFilter>,
    pub chart_data: Vec<ChartPoint>,
    #[schema(example = "2026.05.21")]
    pub exercise_date: String,
    pub exercise_items: Vec<ExerciseItem>,
    pub diary_entries: Vec<DiaryEntry>,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct RecordHighlight {
    #[schema(example = "body")]
    pub id: &'static str,
    #[schema(example = "BODY RECORD")]
    pub title: &'static str,
    #[schema(example = "自分のカラダの記録")]
    pub subtitle: &'static str,
    #[schema(example = "./画像/MyRecommend-1.jpg")]
    pub image: &'static str,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct RecordChartFilter {
    #[schema(example = "day")]
    pub id: &'static str,
    #[schema(example = "日")]
    pub label: &'static str,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseItem {
    #[schema(example = "家事全般（立位・軽い）")]
    pub name: String,
    #[schema(example = 26)]
    pub kcal: i32,
    #[schema(example = "10 min")]
    pub duration: String,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct DiaryEntry {
    #[schema(example = "2026.05.21")]
    pub date: String,
    #[schema(example = "23:25")]
    pub time: String,
    #[schema(example = "朝の散歩で気分が整った。食事は野菜中心で、夜は軽いストレッチを10分。")]
    pub content: String,
}