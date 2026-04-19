use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::dto::auth::{
    LoginMemberRequest, LoginMemberResponse,
    RegisterMemberRequest, RegisterMemberResponse, RegisteredMember,
    GoogleAuthCallbackQuery, GoogleAuthCallbackResponse, GoogleAuthStartRequest,
    GoogleAuthStartResponse,
};
use crate::errors::{AppError, AppResult};
use crate::repositories;

pub async fn register_member(
    pool: &PgPool,
    request: RegisterMemberRequest,
) -> AppResult<RegisterMemberResponse> {
    let email = normalize_email(&request.email)?;
    let full_name = normalize_full_name(&request.full_name)?;
    validate_password(&request.password)?;

    let avatar_url = request.avatar_url.and_then(|value| {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    });

    let created_member = repositories::auth::create_member(
        pool,
        repositories::auth::NewMemberRecord {
            email,
            full_name,
            password_hash: hash_password(&request.password),
            avatar_url,
        },
    )
    .await
    .map_err(map_registration_error)?;

    Ok(RegisterMemberResponse {
        status: "registered".to_string(),
        member: RegisteredMember {
            id: created_member.id,
            email: created_member.email,
            full_name: created_member.full_name,
            is_verified: created_member.is_verified,
            avatar_url: created_member.avatar_url,
            created_at: created_member.created_at,
        },
        settings_initialized: true,
        message: "Member registration completed successfully.".to_string(),
    })
}

pub async fn login_member(
    pool: &PgPool,
    request: LoginMemberRequest,
) -> AppResult<LoginMemberResponse> {
    let email = normalize_email(&request.email)?;
    validate_password(&request.password)?;

    let member = repositories::auth::login_member(pool, &email, &hash_password(&request.password)).await?;

    match member {
        Some(member) => Ok(LoginMemberResponse {
            status: "authenticated".to_string(),
            member: RegisteredMember {
                id: member.id,
                email: member.email,
                full_name: member.full_name,
                is_verified: member.is_verified,
                avatar_url: member.avatar_url,
                created_at: member.created_at,
            },
            message: "Member login completed successfully.".to_string(),
        }),
        None => Err(AppError::Validation("Invalid email or password.".to_string())),
    }
}

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

fn normalize_email(value: &str) -> AppResult<String> {
    let normalized = value.trim().to_ascii_lowercase();

    if normalized.len() < 5 || !normalized.contains('@') || normalized.starts_with('@') || normalized.ends_with('@') {
        return Err(AppError::Validation("Please provide a valid email address.".to_string()));
    }

    Ok(normalized)
}

fn normalize_full_name(value: &str) -> AppResult<String> {
    let normalized = value.trim();

    if normalized.len() < 2 {
        return Err(AppError::Validation("Full name must contain at least 2 characters.".to_string()));
    }

    Ok(normalized.to_string())
}

fn validate_password(value: &str) -> AppResult<()> {
    let trimmed = value.trim();

    if trimmed.len() < 8 {
        return Err(AppError::Validation("Password must contain at least 8 characters.".to_string()));
    }

    Ok(())
}

fn hash_password(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

fn map_registration_error(error: AppError) -> AppError {
    match error {
        AppError::Database(sqlx::Error::Database(database_error))
            if database_error.code().as_deref() == Some("23505") =>
        {
            AppError::Validation("This email is already registered.".to_string())
        }
        other => other,
    }
}