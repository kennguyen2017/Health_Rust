use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct TopPageQuery {
    #[param(example = 3)]
    pub user_id: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TopPageResponse {
    pub summary: TopSummary,
    pub meal_cards: Vec<MealCard>,
    pub top_actions: Vec<TopAction>,
    pub chart_data: Vec<ChartPoint>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TopSummary {
    #[schema(example = "05/21")]
    pub date: String,
    #[schema(example = 75)]
    pub achievement_rate: i32,
    #[schema(example = 230)]
    pub burned_kcal: i32,
    #[schema(example = 60)]
    pub exercise_minutes: i32,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MealCard {
    #[schema(example = "dinner")]
    pub id: String,
    #[schema(example = "Dinner")]
    pub time_label: String,
    #[schema(example = "./画像/d01.jpg")]
    pub image: String,
    #[schema(example = "05.21.Dinner")]
    pub stamp: String,
    #[schema(example = "Dinner meal")]
    pub name: String,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct TopAction {
    pub id: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct ChartPoint {
    #[schema(example = "5")]
    pub month: String,
    #[schema(example = 65.1)]
    pub weight: f64,
    #[schema(example = 24.6)]
    pub fat: f64,
}