-- Add up migration script here
ALTER TABLE users
ADD COLUMN name varchar(80) collate "case_insensitive" not null,
ADD COLUMN profile_link VARCHAR(80) DEFAULT NULL;
