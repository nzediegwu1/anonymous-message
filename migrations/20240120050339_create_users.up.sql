-- Add up migration script here
CREATE TABLE "users"
(
  id uuid primary key default uuid_generate_v1mc(),
  email varchar(80) collate "case_insensitive" unique not null,
  password varchar not null,
  created_at timestamp not null default now(),
  updated_at timestamp
);
SELECT trigger_updated_at('"users"');