use utoipa::OpenApi;

use crate::dto::auth::{
    GoogleAuthCallbackQuery, GoogleAuthCallbackResponse, GoogleAuthStartRequest,
    GoogleAuthStartResponse,
};
use crate::dto::column::{
    ColumnArticle, ColumnDetailArticle, ColumnDetailResponse, ColumnListQuery, ColumnListResponse,
    ColumnPagination, ColumnTab,
};
use crate::dto::my_record::{
    DiaryEntry, DiaryListResponse, ExerciseItem, ExerciseListResponse, MyRecordQuery,
    MyRecordResponse, RecordChartFilter, RecordHighlight,
};
use crate::dto::top::{ChartPoint, MealCard, TopAction, TopPageResponse, TopSummary};
use crate::errors::ErrorResponse;
use crate::handlers::HealthResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::root,
        crate::handlers::health_check,
        crate::handlers::get_top_page,
        crate::handlers::get_my_record,
        crate::handlers::get_my_record_exercises,
        crate::handlers::get_my_record_diaries,
        crate::handlers::get_columns,
        crate::handlers::get_column_detail,
        crate::handlers::start_google_register,
        crate::handlers::start_google_login,
        crate::handlers::google_auth_callback
    ),
    components(
        schemas(
            HealthResponse,
            ErrorResponse,
            GoogleAuthStartRequest,
            GoogleAuthStartResponse,
            GoogleAuthCallbackQuery,
            GoogleAuthCallbackResponse,
            ColumnListQuery,
            ColumnListResponse,
            ColumnDetailResponse,
            ColumnTab,
            ColumnArticle,
            ColumnDetailArticle,
            ColumnPagination,
            MyRecordQuery,
            MyRecordResponse,
            ExerciseListResponse,
            DiaryListResponse,
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
        (name = "My Record", description = "My Record page API endpoints"),
        (name = "Columns", description = "Column page API endpoints"),
        (name = "Auth", description = "Google SSO skeleton endpoints")
    ),
    info(
        title = "Health Rust Backend API",
        version = "0.1.0",
        description = "OpenAPI documentation for the health backend service."
    )
)]
pub struct ApiDoc;