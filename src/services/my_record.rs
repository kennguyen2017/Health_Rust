use sqlx::PgPool;

use crate::dto::my_record::{
    CreateMyRecordRequest, CreateMyRecordResponse, CreatedBodyRecord, DiaryListResponse,
    ExerciseListResponse, MyRecordResponse, RecordChartFilter, RecordHighlight,
};
use crate::errors::{AppError, AppResult};
use crate::repositories;

const RECORD_HIGHLIGHTS: [RecordHighlight; 3] = [
    RecordHighlight {
        id: "body",
        title: "BODY RECORD",
        subtitle: "自分のカラダの記録",
        image: "./画像/MyRecommend-1.jpg",
    },
    RecordHighlight {
        id: "exercise",
        title: "MY EXERCISE",
        subtitle: "自分の運動の記録",
        image: "./画像/MyRecommend-2.jpg",
    },
    RecordHighlight {
        id: "diary",
        title: "MY DIARY",
        subtitle: "自分の日記",
        image: "./画像/MyRecommend-3.jpg",
    },
];

const RECORD_CHART_FILTERS: [RecordChartFilter; 4] = [
    RecordChartFilter { id: "day", label: "日" },
    RecordChartFilter { id: "week", label: "週" },
    RecordChartFilter { id: "month", label: "月" },
    RecordChartFilter { id: "year", label: "年" },
];

pub async fn get_my_record(pool: &PgPool, user_id: i64) -> AppResult<MyRecordResponse> {
    let chart_date = repositories::my_record::fetch_chart_date(pool, user_id).await?;
    let chart_data = repositories::my_record::fetch_chart_data(pool, user_id).await?;
    let exercises = get_my_record_exercises(pool, user_id).await?;
    let diaries = get_my_record_diaries(pool, user_id).await?;

    Ok(MyRecordResponse {
        highlights: RECORD_HIGHLIGHTS.to_vec(),
        chart_date,
        chart_filters: RECORD_CHART_FILTERS.to_vec(),
        chart_data,
        exercise_date: exercises.exercise_date,
        exercise_items: exercises.exercise_items,
        diary_entries: diaries.diary_entries,
    })
}

pub async fn get_my_record_exercises(pool: &PgPool, user_id: i64) -> AppResult<ExerciseListResponse> {
    let exercise_date = repositories::my_record::fetch_exercise_date(pool, user_id).await?;
    let exercise_items = repositories::my_record::fetch_exercise_items(pool, user_id).await?;

    Ok(ExerciseListResponse {
        exercise_date,
        exercise_items,
    })
}

pub async fn get_my_record_diaries(pool: &PgPool, user_id: i64) -> AppResult<DiaryListResponse> {
    let diary_entries = repositories::my_record::fetch_diary_entries(pool, user_id).await?;

    Ok(DiaryListResponse { diary_entries })
}

pub async fn create_my_record(pool: &PgPool, request: CreateMyRecordRequest) -> AppResult<CreateMyRecordResponse> {
    validate_my_record_request(&request)?;

    let mut tx = pool.begin().await?;
    let recorded_at = request.body_record.recorded_at.clone();

    let body_record = repositories::my_record::create_body_record(
        &mut tx,
        repositories::my_record::NewBodyRecord {
            user_id: request.user_id,
            name: request.body_record.name,
            image_url: request.body_record.image_url,
            recorded_at: recorded_at.clone(),
            weight: request.body_record.weight,
            body_fat_rate: request.body_record.body_fat_rate,
        },
    )
    .await?;

    let exercise_item = match request.exercise {
        Some(exercise) => Some(
            repositories::my_record::create_exercise_record(
                &mut tx,
                repositories::my_record::NewExerciseRecord {
                    user_id: request.user_id,
                    title: exercise.title,
                    performed_at: exercise.performed_at,
                    exercise_type: exercise.exercise_type,
                    calories: exercise.calories,
                    image_url: exercise.image_url,
                },
            )
            .await?,
        ),
        None => None,
    };

    let diary_entry = match request.diary {
        Some(diary) => Some(
            repositories::my_record::create_diary_record(
                &mut tx,
                repositories::my_record::NewDiaryRecord {
                    user_id: request.user_id,
                    title: diary.title,
                    content: diary.content,
                    created_at: recorded_at,
                    image_url: diary.image_url,
                },
            )
            .await?,
        ),
        None => None,
    };

    tx.commit().await?;

    Ok(CreateMyRecordResponse {
        body_record: CreatedBodyRecord {
            id: body_record.id,
            chart_date: body_record.chart_date,
            weight: body_record.weight,
            body_fat_rate: body_record.body_fat_rate,
        },
        exercise_item,
        diary_entry,
    })
}

fn validate_my_record_request(request: &CreateMyRecordRequest) -> AppResult<()> {
    if request.user_id <= 0 {
        return Err(AppError::Validation("userId must be greater than 0".to_string()));
    }

    if request.body_record.name.trim().is_empty() {
        return Err(AppError::Validation("bodyRecord.name is required".to_string()));
    }

    if request.body_record.recorded_at.trim().is_empty() {
        return Err(AppError::Validation("bodyRecord.recordedAt is required".to_string()));
    }

    if request.body_record.weight <= 0.0 {
        return Err(AppError::Validation("bodyRecord.weight must be greater than 0".to_string()));
    }

    if request.body_record.body_fat_rate <= 0.0 {
        return Err(AppError::Validation("bodyRecord.bodyFatRate must be greater than 0".to_string()));
    }

    if let Some(exercise) = request.exercise.as_ref() {
        if exercise.title.trim().is_empty() {
            return Err(AppError::Validation("exercise.title is required".to_string()));
        }

        if exercise.exercise_type.trim().is_empty() {
            return Err(AppError::Validation("exercise.exerciseType is required".to_string()));
        }

        if exercise.performed_at.trim().is_empty() {
            return Err(AppError::Validation("exercise.performedAt is required".to_string()));
        }

        if exercise.calories <= 0 {
            return Err(AppError::Validation("exercise.calories must be greater than 0".to_string()));
        }
    }

    if let Some(diary) = request.diary.as_ref() {
        if diary.title.trim().is_empty() {
            return Err(AppError::Validation("diary.title is required".to_string()));
        }

        if diary.content.trim().is_empty() {
            return Err(AppError::Validation("diary.content is required".to_string()));
        }
    }

    Ok(())
}