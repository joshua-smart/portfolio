-- Add up migration script here
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER primary key not null,
    name TEXT not null,
    source_id INTEGER not null
);

CREATE TABLE IF NOT EXISTS tools (
    id INTEGER primary key not null,
    name TEXT not null,
    link TEXT not null
);

CREATE TABLE IF NOT EXISTS project_tools (
    project_id INTEGER not null,
    tool_id INTEGER not null
);

CREATE TABLE IF NOT EXISTS sources (
    id INTEGER primary key not null,
    type TEXT not null,
    link TEXT not null
);

CREATE TABLE IF NOT EXISTS blogs (
    id INTEGER primary key not null,
    title TEXT not null,
    content TEXT not null,
    written TEXT not null
);
