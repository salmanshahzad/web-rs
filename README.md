# Rust Web Server

-   Web server written in Rust using Axum.
-   Connects to PostgreSQL and Redis.
-   Includes middleware for CORS and cookie-based sessions.
-   Serves static files from the `public` directory.

## Usage

### Local

Ensure environment variables are set in `.env`.
See required environment variables in `.env.example`.

The `sqlx` CLI is needed to run migrations before building.

```bash
sqlx migrate run
RUST_LOG=info cargo run
```

### Docker

Ensure environment variables are set in `.env.docker`.

```bash
docker compose up
```

## Endpoints

-   `GET /api/health` returns `204`
-   `GET /api/user` return signed in user
-   `POST /api/user` create new user
-   `PUT /api/user/username` change username
-   `PUT /api/user/password` change password
-   `DELETE /api/user` delete signed in user
-   `POST /api/session` sign in
-   `DELETE /api/session` sign out
