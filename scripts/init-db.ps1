param(
    [switch]$Recreate
)

$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent $PSScriptRoot
$composeFile = Join-Path $repoRoot 'docker-compose.yml'
$migrationDir = Join-Path $repoRoot 'migrations'
$envFile = if (Test-Path (Join-Path $repoRoot '.env')) {
    Join-Path $repoRoot '.env'
} else {
    Join-Path $repoRoot '.env.example'
}
$composeArgs = @('-p', 'health-rust-backend', '-f', $composeFile)

function Get-EnvValues {
    param([string]$Path)

    $values = @{}
    foreach ($line in Get-Content $Path) {
        $trimmed = $line.Trim()
        if (-not $trimmed -or $trimmed.StartsWith('#')) {
            continue
        }

        $parts = $trimmed -split '=', 2
        if ($parts.Count -eq 2) {
            $values[$parts[0]] = $parts[1]
        }
    }

    return $values
}

if (-not (Test-Path $composeFile)) {
    throw "docker-compose.yml was not found at $composeFile"
}

if (-not (Test-Path $migrationDir)) {
    throw "migrations directory was not found at $migrationDir"
}

if (-not (Test-Path $envFile)) {
    throw ".env.example was not found at $envFile"
}

$envValues = Get-EnvValues -Path $envFile
$dbUser = if ($envValues.ContainsKey('POSTGRES_USER')) { $envValues['POSTGRES_USER'] } else { 'postgres' }
$dbPassword = if ($envValues.ContainsKey('POSTGRES_PASSWORD')) { $envValues['POSTGRES_PASSWORD'] } else { 'postgres' }
$dbName = if ($envValues.ContainsKey('POSTGRES_DB')) { $envValues['POSTGRES_DB'] } else { 'health_rust_backend' }
$dbPort = if ($envValues.ContainsKey('POSTGRES_HOST_PORT')) { $envValues['POSTGRES_HOST_PORT'] } else { '5433' }
$databaseUrl = if ($envValues.ContainsKey('DATABASE_URL')) { $envValues['DATABASE_URL'] } else { "postgres://${dbUser}:${dbPassword}@localhost:${dbPort}/${dbName}" }

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
        $isReady = docker compose @composeArgs exec -T db pg_isready -U $dbUser -d $dbName 2>$null
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
            & docker compose @composeArgs exec -T -e PGPASSWORD=$dbPassword db `
                psql -v ON_ERROR_STOP=1 -U $dbUser -d $dbName -f -

        if ($LASTEXITCODE -ne 0) {
            throw "Failed to apply migration $($migrationFile.Name)"
        }
    }

    Write-Host "Database is ready at $databaseUrl"
}
finally {
    Pop-Location
}