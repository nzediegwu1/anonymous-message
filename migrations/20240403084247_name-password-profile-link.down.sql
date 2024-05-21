-- Add down migration script here
ALTER TABLE users
DROP COLUMN name,
DROP COLUMN profile_link;
