use std::collections::BTreeMap;

use sqlx::{PgPool, Row};

use crate::dto::my_record::{DiaryEntry, ExerciseItem};
use crate::dto::top::ChartPoint;
use crate::errors::AppResult;

pub async fn fetch_chart_date(pool: &PgPool, user_id: i64) -> AppResult<String> {
    let row = sqlx::query(
        r#"
        SELECT COALESCE(TO_CHAR(MAX(recorded_at), 'YYYY.MM.DD'), TO_CHAR(CURRENT_DATE, 'YYYY.MM.DD')) AS chart_date
        FROM body_records
        WHERE user_id = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(row.get("chart_date"))
}

pub async fn fetch_exercise_date(pool: &PgPool, user_id: i64) -> AppResult<String> {
    let row = sqlx::query(
        r#"
        SELECT COALESCE(TO_CHAR(MAX(performed_at), 'YYYY.MM.DD'), TO_CHAR(CURRENT_DATE, 'YYYY.MM.DD')) AS exercise_date
        FROM exercise_records
        WHERE user_id = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(row.get("exercise_date"))
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

pub async fn fetch_exercise_items(pool: &PgPool, user_id: i64) -> AppResult<Vec<ExerciseItem>> {
    let rows = sqlx::query(
        r#"
        WITH latest_exercise_day AS (
            SELECT MAX(performed_at::date) AS day
            FROM exercise_records
            WHERE user_id = $1 AND deleted_at IS NULL
        )
        SELECT title, calories, exercise_type
        FROM exercise_records
        WHERE user_id = $1
            AND deleted_at IS NULL
            AND performed_at::date = (SELECT day FROM latest_exercise_day)
        ORDER BY performed_at ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| ExerciseItem {
            name: row.get("title"),
            kcal: row.get::<i16, _>("calories") as i32,
            duration: row.get("exercise_type"),
        })
        .collect())
}

pub async fn fetch_diary_entries(pool: &PgPool, user_id: i64) -> AppResult<Vec<DiaryEntry>> {
    let rows = sqlx::query(
        r#"
        SELECT TO_CHAR(created_at, 'YYYY.MM.DD') AS entry_date,
               TO_CHAR(created_at, 'HH24:MI') AS entry_time,
               content
        FROM diaries
        WHERE user_id = $1 AND deleted_at IS NULL
        ORDER BY created_at DESC
        LIMIT 8
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| DiaryEntry {
            date: row.get("entry_date"),
            time: row.get("entry_time"),
            content: row.get("content"),
        })
        .collect())
}