param(
    [switch]$Recreate
)

$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent $PSScriptRoot
$composeFile = Join-Path $repoRoot 'docker-compose.yml'
$migrationDir = Join-Path $repoRoot 'migrations'
$composeArgs = @('-p', 'health-rust-backend', '-f', $composeFile)

if (-not (Test-Path $composeFile)) {
    throw "docker-compose.yml was not found at $composeFile"
}

if (-not (Test-Path $migrationDir)) {
    throw "migrations directory was not found at $migrationDir"
}

docker info *> $null
if ($LASTEXITCODE -ne 0) {
    throw 'Docker Desktop is not running. Start Docker Desktop first, then rerun .\scripts\init-db.ps1.'
}

Push-Location $repoRoot
try {
    if ($Recreate) {
        docker compose @composeArgs down -v
    }

    docker compose @composeArgs up -d db

    $attempt = 0
    do {
        $attempt += 1
        $isReady = docker compose @composeArgs exec -T db pg_isready -U postgres -d health_rust_backend 2>$null
        if ($LASTEXITCODE -eq 0) {
            break
        }

        if ($attempt -ge 30) {
            throw 'Postgres container did not become ready in time.'
        }

        Start-Sleep -Seconds 2
    } while ($true)

    $migrationFiles = Get-ChildItem -Path $migrationDir -Filter '*.up.sql' | Sort-Object Name

    foreach ($migrationFile in $migrationFiles) {
        Write-Host "Applying migration $($migrationFile.Name)..."
        Get-Content -Raw $migrationFile.FullName |
            & docker compose @composeArgs exec -T -e PGPASSWORD=postgres db `
                psql -v ON_ERROR_STOP=1 -U postgres -d health_rust_backend -f -

        if ($LASTEXITCODE -ne 0) {
            throw "Failed to apply migration $($migrationFile.Name)"
        }
    }

    Write-Host 'Database is ready at postgres://postgres:postgres@localhost:5433/health_rust_backend'
}
finally {
    Pop-Location
}