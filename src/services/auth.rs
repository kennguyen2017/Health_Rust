use std::time::{SystemTime, UNIX_EPOCH};

use crate::dto::auth::{
    GoogleAuthCallbackQuery, GoogleAuthCallbackResponse, GoogleAuthStartRequest,
    GoogleAuthStartResponse,
};
use crate::errors::AppResult;

pub async fn start_google_register(
    request: GoogleAuthStartRequest,
) -> AppResult<GoogleAuthStartResponse> {
    Ok(build_start_response("register", request))
}

pub async fn start_google_login(request: GoogleAuthStartRequest) -> AppResult<GoogleAuthStartResponse> {
    Ok(build_start_response("login", request))
}

pub async fn handle_google_callback(
    query: GoogleAuthCallbackQuery,
) -> AppResult<GoogleAuthCallbackResponse> {
    let has_callback_payload = query.code.is_some() || query.state.is_some();
    let message = if has_callback_payload {
        "Google callback payload reached backend skeleton. Token exchange is not enabled yet."
    } else {
        "Google callback endpoint is live. Redirect provider params here when OAuth is wired."
    };

    Ok(GoogleAuthCallbackResponse {
        provider: "google".to_string(),
        status: if has_callback_payload {
            "callback-received".to_string()
        } else {
            "callback-ready".to_string()
        },
        intent: query.intent,
        state: query.state,
        code: query.code,
        scope: query.scope,
        authuser: query.authuser,
        prompt: query.prompt,
        message: message.to_string(),
    })
}

fn build_start_response(intent: &str, request: GoogleAuthStartRequest) -> GoogleAuthStartResponse {
    let state = build_state(intent);
    let authorization_url = request
        .redirect_uri
        .as_ref()
        .map(|redirect_uri| build_placeholder_redirect(redirect_uri, intent, &state));

    GoogleAuthStartResponse {
        provider: "google".to_string(),
        intent: intent.to_string(),
        status: "skeleton-ready".to_string(),
        state: state.clone(),
        authorization_url,
        redirect_uri: request.redirect_uri,
        message: format!(
            "Google {intent} skeleton is connected. Real OAuth exchange is intentionally disabled."
        ),
    }
}

fn build_state(intent: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    format!("google-{intent}-{timestamp}")
}

fn build_placeholder_redirect(redirect_uri: &str, intent: &str, state: &str) -> String {
    let separator = if redirect_uri.contains('?') { '&' } else { '?' };
    format!("{redirect_uri}{separator}provider=google&intent={intent}&state={state}")
}