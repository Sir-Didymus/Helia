--  Migration 1: Initial migration

CREATE TABLE action (
    id BLOB PRIMARY KEY,
    title TEXT NOT NULL,
    created_at DATETIME NOT NULL
);

PRAGMA user_version = 1;

