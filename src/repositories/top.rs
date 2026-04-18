use std::collections::BTreeMap;

use sqlx::{PgPool, Row};

use crate::dto::top::{ChartPoint, MealCard, TopSummary};
use crate::errors::AppResult;

pub async fn fetch_summary(pool: &PgPool, user_id: i64) -> AppResult<TopSummary> {
    let row = sqlx::query(
        r#"
        WITH latest_summary_log AS (
            SELECT options
            FROM user_action_logs
            WHERE user_id = $1
                AND deleted_at IS NULL
                AND action = 'top_page_summary'
            ORDER BY created_at DESC
            LIMIT 1
        ),
        derived_summary AS (
            SELECT
                COALESCE(
                    (SELECT TO_CHAR(MAX(eaten_at), 'MM/DD') FROM meals WHERE user_id = $1 AND deleted_at IS NULL),
                    (SELECT TO_CHAR(MAX(recorded_at), 'MM/DD') FROM body_records WHERE user_id = $1 AND deleted_at IS NULL),
                    TO_CHAR(CURRENT_DATE, 'MM/DD')
                ) AS date,
                COALESCE((
                    WITH latest_meal_day AS (
                        SELECT MAX(eaten_at::date) AS day
                        FROM meals
                        WHERE user_id = $1 AND deleted_at IS NULL
                    )
                    SELECT LEAST(COUNT(DISTINCT meal_type) * 25, 100)
                    FROM meals
                    WHERE user_id = $1
                        AND deleted_at IS NULL
                        AND eaten_at::date = (SELECT day FROM latest_meal_day)
                ), 0)::INT AS achievement_rate,
                COALESCE((
                    WITH latest_exercise_day AS (
                        SELECT MAX(performed_at::date) AS day
                        FROM exercise_records
                        WHERE user_id = $1 AND deleted_at IS NULL
                    )
                    SELECT SUM(calories)
                    FROM exercise_records
                    WHERE user_id = $1
                        AND deleted_at IS NULL
                        AND performed_at::date = (SELECT day FROM latest_exercise_day)
                ), 0)::INT AS burned_kcal,
                COALESCE((
                    WITH latest_exercise_day AS (
                        SELECT MAX(performed_at::date) AS day
                        FROM exercise_records
                        WHERE user_id = $1 AND deleted_at IS NULL
                    )
                    SELECT SUM(NULLIF(REGEXP_REPLACE(exercise_type, '[^0-9]', '', 'g'), '')::INT)
                    FROM exercise_records
                    WHERE user_id = $1
                        AND deleted_at IS NULL
                        AND performed_at::date = (SELECT day FROM latest_exercise_day)
                ), 0)::INT AS exercise_minutes
        )
        SELECT
            COALESCE(
                (SELECT options ->> 'date' FROM latest_summary_log),
                (SELECT date FROM derived_summary)
            ) AS date,
            COALESCE((
                SELECT (options ->> 'achievement_rate')::INT FROM latest_summary_log
            ), (SELECT achievement_rate FROM derived_summary), 0)::INT AS achievement_rate,
            COALESCE((
                SELECT (options ->> 'burned_kcal')::INT FROM latest_summary_log
            ), (SELECT burned_kcal FROM derived_summary), 0)::INT AS burned_kcal,
            COALESCE((
                SELECT (options ->> 'exercise_minutes')::INT FROM latest_summary_log
            ), (SELECT exercise_minutes FROM derived_summary), 0)::INT AS exercise_minutes
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(TopSummary {
        date: row.get("date"),
        achievement_rate: row.get("achievement_rate"),
        burned_kcal: row.get("burned_kcal"),
        exercise_minutes: row.get("exercise_minutes"),
    })
}

pub async fn fetch_chart_data(pool: &PgPool, user_id: i64) -> AppResult<Vec<ChartPoint>> {
    let rows = sqlx::query(
        r#"
        SELECT
            EXTRACT(MONTH FROM recorded_at)::INT AS month,
            ROUND(AVG(weight), 1)::FLOAT8 AS weight,
            ROUND(AVG(body_fat_rate), 1)::FLOAT8 AS fat
        FROM body_records
        WHERE user_id = $1 AND deleted_at IS NULL
        GROUP BY EXTRACT(MONTH FROM recorded_at)
        ORDER BY month
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut month_values = BTreeMap::new();
    for row in rows {
        month_values.insert(
            row.get::<i32, _>("month"),
            (row.get::<f64, _>("weight"), row.get::<f64, _>("fat")),
        );
    }

    let first_known = month_values.values().next().copied().unwrap_or((0.0, 0.0));
    let mut previous = None;
    let mut chart = Vec::with_capacity(12);

    for month in 1..=12 {
        let values = if let Some(current) = month_values.get(&month).copied() {
            previous = Some(current);
            current
        } else {
            previous.unwrap_or(first_known)
        };

        chart.push(ChartPoint {
            month: month.to_string(),
            weight: values.0,
            fat: values.1,
        });
    }

    Ok(chart)
}

pub async fn fetch_meal_cards(pool: &PgPool, user_id: i64) -> AppResult<Vec<MealCard>> {
    let rows = sqlx::query(
        r#"
        WITH latest_meal_day AS (
            SELECT MAX(eaten_at::date) AS day
            FROM meals
            WHERE user_id = $1 AND deleted_at IS NULL
        )
        SELECT
            meal_type,
            name,
            image_url,
            TO_CHAR(eaten_at, 'MM.DD') AS stamp_day
        FROM meals
        WHERE user_id = $1
            AND deleted_at IS NULL
            AND eaten_at::date = (SELECT day FROM latest_meal_day)
        ORDER BY CASE meal_type
            WHEN 'morning' THEN 1
            WHEN 'lunch' THEN 2
            WHEN 'dinner' THEN 3
            WHEN 'snack' THEN 4
            ELSE 5
        END
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let meal_cards = rows
        .into_iter()
        .map(|row| {
            let meal_type = row.get::<String, _>("meal_type");
            let time_label = title_case(&meal_type);
            let stamp_day = row.get::<String, _>("stamp_day");

            MealCard {
                id: meal_type.clone(),
                time_label: time_label.clone(),
                image: row
                    .get::<Option<String>, _>("image_url")
                    .unwrap_or_else(String::new),
                stamp: format!("{stamp_day}.{time_label}"),
                name: row.get("name"),
            }
        })
        .collect();

    Ok(meal_cards)
}

fn title_case(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
        None => String::new(),
    }
}