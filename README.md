# Rust Web Server

- Web server written in Rust using Axum.
- Connects to PostgreSQL and Redis.
- Includes middleware for CORS as well as cookie and JWT authentication.
- Serves static files from the `public` directory.

## Environment Variables

- `DATABASE_URL`
- `JWT_SECRET`
- `PASSWORD_SALT`
- `PORT`
- `REDIS_URL`

## Endpoints

- `GET /api/health` returns `204`
- `GET /api/user` return signed in user
- `POST /api/user` create new user
- `PUT /api/user/username` change username
- `PUT /api/user/password` change password
- `DELETE /api/user` delete signed in user
- `POST /api/session` sign in
- `DELETE /api/session` sign out

## Database

Ensure the database has a `user` table.

`CREATE TABLE "user" (id SERIAL, username TEXT NOT NULL UNIQUE, password TEXT NOT NULL);`
