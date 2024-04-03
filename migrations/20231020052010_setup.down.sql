-- Add down migration script here
DROP COLLATION case_insensitive;
DROP extension IF EXISTS "uuid-ossp";