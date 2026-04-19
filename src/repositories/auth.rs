use sqlx::{postgres::PgRow, PgPool, Row};

use crate::errors::AppResult;

#[derive(Debug, Clone)]
pub struct NewMemberRecord {
    pub email: String,
    pub full_name: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RegisteredMemberRecord {
    pub id: i64,
    pub email: String,
    pub full_name: String,
    pub is_verified: bool,
    pub avatar_url: Option<String>,
    pub created_at: String,
}

pub async fn create_member(pool: &PgPool, new_member: NewMemberRecord) -> AppResult<RegisteredMemberRecord> {
    let mut transaction = pool.begin().await?;

    let row = sqlx::query(
        r#"
        INSERT INTO users (
            email,
            full_name,
            password_hash,
            previous_password,
            last_login,
            is_admin,
            is_verified,
            is_password_reset,
            login_attempts,
            last_updated_password,
            avatar_url,
            reference_id
        )
        VALUES ($1, $2, $3, NULL, NULL, FALSE, FALSE, FALSE, 0, NOW(), $4, NULL)
        RETURNING
            id,
            email,
            full_name,
            is_verified,
            avatar_url,
            TO_CHAR(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS.MS"Z"') AS created_at
        "#,
    )
    .bind(new_member.email)
    .bind(new_member.full_name)
    .bind(new_member.password_hash)
    .bind(new_member.avatar_url)
    .fetch_one(&mut *transaction)
    .await?;

    let member = map_registered_member_row(row);

    sqlx::query(
        r#"
        INSERT INTO settings (user_id, created_by, updated_by)
        VALUES ($1, $1, $1)
        ON CONFLICT (user_id) DO NOTHING
        "#,
    )
    .bind(member.id)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(member)
}

fn map_registered_member_row(row: PgRow) -> RegisteredMemberRecord {
    RegisteredMemberRecord {
        id: row.get("id"),
        email: row.get("email"),
        full_name: row.get("full_name"),
        is_verified: row.get("is_verified"),
        avatar_url: row.get("avatar_url"),
        created_at: row.get("created_at"),
    }
}