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

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseListResponse {
    #[schema(example = "2026.05.21")]
    pub exercise_date: String,
    pub exercise_items: Vec<ExerciseItem>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiaryListResponse {
    pub diary_entries: Vec<DiaryEntry>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateMyRecordRequest {
    #[schema(example = 3)]
    pub user_id: i64,
    pub body_record: CreateBodyRecordInput,
    pub exercise: Option<CreateExerciseInput>,
    pub diary: Option<CreateDiaryInput>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBodyRecordInput {
    #[schema(example = "Body Record")]
    pub name: String,
    #[schema(example = "2026-04-19T07:30:00Z")]
    pub recorded_at: String,
    #[schema(example = 68.4)]
    pub weight: f64,
    #[schema(example = 18.7)]
    pub body_fat_rate: f64,
    #[schema(example = "https://images.unsplash.com/photo-1517836357463-d25dfeac3438")]
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateExerciseInput {
    #[schema(example = "Morning Walk")]
    pub title: String,
    #[schema(example = "20 min")]
    pub exercise_type: String,
    #[schema(example = 120)]
    pub calories: i32,
    #[schema(example = "2026-04-19T07:45:00Z")]
    pub performed_at: String,
    #[schema(example = "https://images.unsplash.com/photo-1517838277536-f5f99be501cd")]
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateDiaryInput {
    #[schema(example = "Morning reflection")]
    pub title: String,
    #[schema(example = "Walked for 20 minutes and prepared a lighter breakfast.")]
    pub content: String,
    #[schema(example = "https://images.unsplash.com/photo-1517842645767-c639042777db")]
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateMyRecordResponse {
    pub body_record: CreatedBodyRecord,
    pub exercise_item: Option<ExerciseItem>,
    pub diary_entry: Option<DiaryEntry>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBodyRecord {
    #[schema(example = 101)]
    pub id: i64,
    #[schema(example = "2026.04.19")]
    pub chart_date: String,
    #[schema(example = 68.4)]
    pub weight: f64,
    #[schema(example = 18.7)]
    pub body_fat_rate: f64,
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