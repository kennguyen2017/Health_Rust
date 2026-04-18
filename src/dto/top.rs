use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TopPageQuery {
    pub user_id: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopPageResponse {
    pub summary: TopSummary,
    pub meal_cards: Vec<MealCard>,
    pub top_actions: Vec<TopAction>,
    pub chart_data: Vec<ChartPoint>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopSummary {
    pub date: String,
    pub achievement_rate: i32,
    pub burned_kcal: i32,
    pub exercise_minutes: i32,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MealCard {
    pub id: String,
    pub time_label: String,
    pub image: String,
    pub stamp: String,
    pub name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct TopAction {
    pub id: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
}

#[derive(Debug, Serialize, Clone)]
pub struct ChartPoint {
    pub month: String,
    pub weight: f64,
    pub fat: f64,
}