use std::collections::BTreeMap;

use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::dto::my_record::{DiaryEntry, ExerciseItem};
use crate::dto::top::ChartPoint;
use crate::errors::AppResult;

#[derive(Debug, Clone)]
pub struct NewBodyRecord {
    pub user_id: i64,
    pub name: String,
    pub image_url: Option<String>,
    pub recorded_at: String,
    pub weight: f64,
    pub body_fat_rate: f64,
}

#[derive(Debug, Clone)]
pub struct CreatedBodyRecordRow {
    pub id: i64,
    pub chart_date: String,
    pub weight: f64,
    pub body_fat_rate: f64,
}

#[derive(Debug, Clone)]
pub struct NewExerciseRecord {
    pub user_id: i64,
    pub title: String,
    pub performed_at: String,
    pub exercise_type: String,
    pub calories: i32,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewDiaryRecord {
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub image_url: Option<String>,
}

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

pub async fn create_body_record(
    tx: &mut Transaction<'_, Postgres>,
    new_record: NewBodyRecord,
) -> AppResult<CreatedBodyRecordRow> {
    let row = sqlx::query(
        r#"
        INSERT INTO body_records (
            user_id,
            name,
            image_url,
            recorded_at,
            weight,
            body_fat_rate,
            created_by,
            updated_by
        )
        VALUES ($1, $2, $3, $4::timestamptz, $5, $6, $1, $1)
        RETURNING
            id,
            TO_CHAR(recorded_at, 'YYYY.MM.DD') AS chart_date,
            weight::FLOAT8 AS weight,
            body_fat_rate::FLOAT8 AS body_fat_rate
        "#,
    )
    .bind(new_record.user_id)
    .bind(new_record.name)
    .bind(new_record.image_url)
    .bind(new_record.recorded_at)
    .bind(new_record.weight)
    .bind(new_record.body_fat_rate)
    .fetch_one(&mut **tx)
    .await?;

    Ok(CreatedBodyRecordRow {
        id: row.get("id"),
        chart_date: row.get("chart_date"),
        weight: row.get("weight"),
        body_fat_rate: row.get("body_fat_rate"),
    })
}

pub async fn create_exercise_record(
    tx: &mut Transaction<'_, Postgres>,
    new_record: NewExerciseRecord,
) -> AppResult<ExerciseItem> {
    let row = sqlx::query(
        r#"
        INSERT INTO exercise_records (
            user_id,
            title,
            performed_at,
            exercise_type,
            calories,
            image_url,
            created_by,
            updated_by
        )
        VALUES ($1, $2, $3::timestamptz, $4, $5, $6, $1, $1)
        RETURNING title, calories, exercise_type
        "#,
    )
    .bind(new_record.user_id)
    .bind(new_record.title)
    .bind(new_record.performed_at)
    .bind(new_record.exercise_type)
    .bind(new_record.calories)
    .bind(new_record.image_url)
    .fetch_one(&mut **tx)
    .await?;

    Ok(ExerciseItem {
        name: row.get("title"),
        kcal: row.get::<i16, _>("calories") as i32,
        duration: row.get("exercise_type"),
    })
}

pub async fn create_diary_record(
    tx: &mut Transaction<'_, Postgres>,
    new_record: NewDiaryRecord,
) -> AppResult<DiaryEntry> {
    let row = sqlx::query(
        r#"
        INSERT INTO diaries (
            user_id,
            title,
            content,
            image_url,
            created_by,
            updated_by,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $1, $1, $5::timestamptz, $5::timestamptz)
        RETURNING
            TO_CHAR(created_at, 'YYYY.MM.DD') AS entry_date,
            TO_CHAR(created_at, 'HH24:MI') AS entry_time,
            content
        "#,
    )
    .bind(new_record.user_id)
    .bind(new_record.title)
    .bind(new_record.content)
    .bind(new_record.image_url)
    .bind(new_record.created_at)
    .fetch_one(&mut **tx)
    .await?;

    Ok(DiaryEntry {
        date: row.get("entry_date"),
        time: row.get("entry_time"),
        content: row.get("content"),
    })
}