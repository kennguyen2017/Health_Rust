CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    full_name VARCHAR(255) NOT NULL,
    password_hash VARCHAR(64) NOT NULL,
    previous_password TEXT,
    last_login TIMESTAMPTZ,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    is_password_reset BOOLEAN NOT NULL DEFAULT FALSE,
    login_attempts INTEGER NOT NULL DEFAULT 0,
    last_updated_password TIMESTAMPTZ,
    avatar_url TEXT,
    reference_id BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE courses (
    id BIGSERIAL PRIMARY KEY,
    course_type VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE columns (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    image_url TEXT,
    content TEXT NOT NULL,
    created_by BIGINT REFERENCES users(id),
    updated_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE settings (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    created_by BIGINT REFERENCES users(id),
    updated_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE body_records (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    image_url TEXT,
    recorded_at TIMESTAMPTZ NOT NULL,
    weight NUMERIC(5,2) NOT NULL,
    body_fat_rate NUMERIC(5,2) NOT NULL,
    created_by BIGINT REFERENCES users(id),
    updated_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE day_charts (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    weight NUMERIC(5,2) NOT NULL,
    body_fat_rate NUMERIC(5,2) NOT NULL
);

CREATE TABLE week_charts (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    weight NUMERIC(5,2) NOT NULL,
    body_fat_rate NUMERIC(5,2) NOT NULL
);

CREATE TABLE month_charts (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    weight NUMERIC(5,2) NOT NULL,
    body_fat_rate NUMERIC(5,2) NOT NULL
);

CREATE TABLE year_charts (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    weight NUMERIC(5,2) NOT NULL,
    body_fat_rate NUMERIC(5,2) NOT NULL
);

CREATE TABLE meals (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    calories NUMERIC(7,2) NOT NULL,
    meal_type VARCHAR(20) NOT NULL CHECK (meal_type IN ('morning', 'lunch', 'dinner', 'snack')),
    eaten_at TIMESTAMPTZ NOT NULL,
    image_url TEXT,
    created_by BIGINT REFERENCES users(id),
    updated_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE diaries (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    image_url TEXT,
    created_by BIGINT REFERENCES users(id),
    updated_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE exercise_records (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    performed_at TIMESTAMPTZ NOT NULL,
    exercise_type VARCHAR(50) NOT NULL,
    calories SMALLINT NOT NULL,
    image_url TEXT,
    created_by BIGINT REFERENCES users(id),
    updated_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE challenges (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id BIGINT REFERENCES courses(id) ON DELETE SET NULL,
    challenge_time TIMESTAMPTZ NOT NULL,
    created_by BIGINT REFERENCES users(id),
    updated_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE user_action_logs (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    action VARCHAR(255) NOT NULL,
    data TEXT,
    options JSONB,
    status VARCHAR(100) NOT NULL,
    created_by BIGINT REFERENCES users(id),
    updated_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE INDEX idx_body_records_user_id_recorded_at ON body_records(user_id, recorded_at DESC);
CREATE INDEX idx_day_charts_user_id_recorded_at ON day_charts(user_id, recorded_at DESC);
CREATE INDEX idx_week_charts_user_id_recorded_at ON week_charts(user_id, recorded_at DESC);
CREATE INDEX idx_month_charts_user_id_recorded_at ON month_charts(user_id, recorded_at DESC);
CREATE INDEX idx_year_charts_user_id_recorded_at ON year_charts(user_id, recorded_at DESC);
CREATE INDEX idx_meals_user_id_eaten_at ON meals(user_id, eaten_at DESC);
CREATE INDEX idx_diaries_user_id_created_at ON diaries(user_id, created_at DESC);
CREATE INDEX idx_exercise_records_user_id_performed_at ON exercise_records(user_id, performed_at DESC);
CREATE INDEX idx_challenges_user_id_challenge_time ON challenges(user_id, challenge_time DESC);
CREATE INDEX idx_user_action_logs_user_id_created_at ON user_action_logs(user_id, created_at DESC);
CREATE INDEX idx_columns_created_at ON columns(created_at DESC);
CREATE INDEX idx_user_email ON users(email);