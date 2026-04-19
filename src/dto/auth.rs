use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterMemberRequest {
    #[schema(example = "linh.nguyen@example.com")]
    pub email: String,
    #[schema(example = "Linh Nguyen")]
    pub full_name: String,
    #[schema(example = "StrongPass123")]
    pub password: String,
    #[schema(example = "https://images.unsplash.com/photo-1494790108377-be9c29b29330")]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredMember {
    #[schema(example = 3)]
    pub id: i64,
    #[schema(example = "linh.nguyen@example.com")]
    pub email: String,
    #[schema(example = "Linh Nguyen")]
    pub full_name: String,
    #[schema(example = false)]
    pub is_verified: bool,
    #[schema(example = "https://images.unsplash.com/photo-1494790108377-be9c29b29330")]
    pub avatar_url: Option<String>,
    #[schema(example = "2026-04-19T06:15:42.000Z")]
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterMemberResponse {
    #[schema(example = "registered")]
    pub status: String,
    pub member: RegisteredMember,
    #[schema(example = true)]
    pub settings_initialized: bool,
    #[schema(example = "Member registration completed successfully.")]
    pub message: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GoogleAuthStartRequest {
    #[schema(example = "http://localhost:5173/#/auth")]
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GoogleAuthStartResponse {
    #[schema(example = "google")]
    pub provider: String,
    #[schema(example = "register")]
    pub intent: String,
    #[schema(example = "skeleton-ready")]
    pub status: String,
    #[schema(example = "google-register-1777777777777")]
    pub state: String,
    #[schema(example = "http://localhost:5173/#/auth?provider=google&intent=register&state=google-register-1777777777777")]
    pub authorization_url: Option<String>,
    #[schema(example = "http://localhost:5173/#/auth")]
    pub redirect_uri: Option<String>,
    #[schema(example = "Google register skeleton is connected. Real OAuth exchange is intentionally disabled.")]
    pub message: String,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct GoogleAuthCallbackQuery {
    #[param(example = "4/0AX4XfWh-example")]
    pub code: Option<String>,
    #[param(example = "google-register-1777777777777")]
    pub state: Option<String>,
    #[param(example = "openid email profile")]
    pub scope: Option<String>,
    #[param(example = "0")]
    pub authuser: Option<String>,
    #[param(example = "consent")]
    pub prompt: Option<String>,
    #[param(example = "register")]
    pub intent: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GoogleAuthCallbackResponse {
    #[schema(example = "google")]
    pub provider: String,
    #[schema(example = "callback-received")]
    pub status: String,
    #[schema(example = "register")]
    pub intent: Option<String>,
    #[schema(example = "google-register-1777777777777")]
    pub state: Option<String>,
    #[schema(example = "4/0AX4XfWh-example")]
    pub code: Option<String>,
    #[schema(example = "openid email profile")]
    pub scope: Option<String>,
    #[schema(example = "0")]
    pub authuser: Option<String>,
    #[schema(example = "consent")]
    pub prompt: Option<String>,
    #[schema(example = "Google callback payload reached backend skeleton. Token exchange is not enabled yet.")]
    pub message: String,
}