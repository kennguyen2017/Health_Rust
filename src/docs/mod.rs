use utoipa::OpenApi;

use crate::dto::my_record::{DiaryEntry, ExerciseItem, MyRecordQuery, MyRecordResponse, RecordChartFilter, RecordHighlight};
use crate::dto::top::{ChartPoint, MealCard, TopAction, TopPageResponse, TopSummary};
use crate::errors::ErrorResponse;
use crate::handlers::HealthResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::root,
        crate::handlers::health_check,
        crate::handlers::get_top_page,
        crate::handlers::get_my_record
    ),
    components(
        schemas(
            HealthResponse,
            ErrorResponse,
            MyRecordQuery,
            MyRecordResponse,
            RecordHighlight,
            RecordChartFilter,
            ExerciseItem,
            DiaryEntry,
            TopPageResponse,
            TopSummary,
            MealCard,
            TopAction,
            ChartPoint
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Top Page", description = "Top page API endpoints"),
        (name = "My Record", description = "My Record page API endpoints")
    ),
    info(
        title = "Health Rust Backend API",
        version = "0.1.0",
        description = "OpenAPI documentation for the health backend service."
    )
)]
pub struct ApiDoc;