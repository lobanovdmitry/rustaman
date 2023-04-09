CREATE TABLE app_user (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW()
);
