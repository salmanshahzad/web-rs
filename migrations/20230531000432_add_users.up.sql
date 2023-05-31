CREATE TABLE "user" (
  id SERIAL,
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL
);
