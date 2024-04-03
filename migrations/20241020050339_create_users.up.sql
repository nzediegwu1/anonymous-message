-- Add up migration script here
CREATE TABLE "users"
(
  id uuid primary key default uuid_generate_v1mc(),
  email text collate "case_insensitive" unique unique not null,
  password text not null,
  created_at timestamp not null default now(),
  updated_at timestamp
);
SELECT trigger_updated_at('"users"');