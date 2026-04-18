use sqlx::PgPool;

use crate::dto::my_record::{MyRecordResponse, RecordChartFilter, RecordHighlight};
use crate::errors::AppResult;
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
    let exercise_date = repositories::my_record::fetch_exercise_date(pool, user_id).await?;
    let exercise_items = repositories::my_record::fetch_exercise_items(pool, user_id).await?;
    let diary_entries = repositories::my_record::fetch_diary_entries(pool, user_id).await?;

    Ok(MyRecordResponse {
        highlights: RECORD_HIGHLIGHTS.to_vec(),
        chart_date,
        chart_filters: RECORD_CHART_FILTERS.to_vec(),
        chart_data,
        exercise_date,
        exercise_items,
        diary_entries,
    })
}