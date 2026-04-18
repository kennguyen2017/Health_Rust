use sqlx::PgPool;

use crate::dto::top::{TopAction, TopPageResponse};
use crate::errors::AppResult;
use crate::repositories;

const TOP_ACTIONS: [TopAction; 4] = [
    TopAction {
        id: "morning",
        title: "Morning",
        icon: "sun",
    },
    TopAction {
        id: "lunch",
        title: "Lunch",
        icon: "meal",
    },
    TopAction {
        id: "dinner",
        title: "Dinner",
        icon: "fork",
    },
    TopAction {
        id: "snack",
        title: "Snack",
        icon: "coffee",
    },
];

pub async fn get_top_page(pool: &PgPool, user_id: i64) -> AppResult<TopPageResponse> {
    let summary = repositories::top::fetch_summary(pool, user_id).await?;
    let meal_cards = repositories::top::fetch_meal_cards(pool, user_id).await?;
    let chart_data = repositories::top::fetch_chart_data(pool, user_id).await?;

    Ok(TopPageResponse {
        summary,
        meal_cards,
        top_actions: TOP_ACTIONS.to_vec(),
        chart_data,
    })
}