-- Add up migration script here
CREATE extension IF NOT EXISTS "uuid-ossp";

CREATE OR REPLACE FUNCTION set_updated_at()
  returns trigger as
$$
begin
  NEW.updated_at = now() at time zone 'UTC';
  return NEW;
end;
$$ language plpgsql;

CREATE OR REPLACE FUNCTION trigger_updated_at(tablename regclass)
  returns void as
$$
begin
  execute format('CREATE TRIGGER set_updated_at
    BEFORE UPDATE
    ON %I
    FOR EACH ROW
    WHEN (OLD is distinct from NEW)
  EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;

CREATE COLLATION case_insensitive (provider = icu, locale = 'und-u-ks-level2');
