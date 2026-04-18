# Local Postgres With Docker

This repository ships with a local PostgreSQL container and SQL migrations that can be applied without installing `sqlx-cli`.

Docker Compose reads `POSTGRES_*` values from `.env` when present. The tracked `.env.example` contains the default local values.

## Prerequisites

- Docker Desktop
- Docker Compose v2
- PowerShell

## Start The Database

Optional: copy `.env.example` to `.env` first if you want to override the local Postgres credentials or host port.

From the backend repository root, run:

```powershell
docker compose up -d db
```

By default the container exposes PostgreSQL on `localhost:5433` with these credentials:

- user: `postgres`
- password: `postgres`
- database: `health_rust_backend`

## Apply Schema And Seed Data

Run the setup script:

```powershell
.\scripts\init-db.ps1
```

If you want a full reset that recreates the volume and reapplies all migrations:

```powershell
.\scripts\init-db.ps1 -Recreate
```

This script will:

1. Start the Postgres container.
2. Wait until the database is ready.
3. Apply every `*.up.sql` file in the `migrations` directory in filename order.

## Verify Seed Data

You can inspect the seeded tables with:

```powershell
docker compose exec -T db psql -U postgres -d health_rust_backend -c "SELECT id, email, full_name FROM users;"
docker compose exec -T db psql -U postgres -d health_rust_backend -c "SELECT id, meal_type, name FROM meals ORDER BY id;"
docker compose exec -T db psql -U postgres -d health_rust_backend -c "SELECT id, title FROM columns ORDER BY id;"
```

## Match The App Environment

The current application connection string in `.env.example` already matches the Docker database:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5433/health_rust_backend
```