-- Add up migration script here
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER primary key not null,
    name TEXT not null,
    description TEXT not null,
    source_type TEXT not null,
    source_link TEXT not null
);

CREATE TABLE IF NOT EXISTS posts (
    id INTEGER primary key not null,
    title TEXT not null,
    content TEXT not null,
    written TEXT not null
);

CREATE TABLE IF NOT EXISTS events (
    id INTEGER primary key not null,
    timestamp TEXT not null,
    description TEXT not null
);
