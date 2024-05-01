BEGIN;

ALTER TABLE public.users
ADD COLUMN lon real DEFAULT 0.0,
ADD COLUMN lat real DEFAULT 0.0;

ALTER TABLE public.users
ADD COLUMN open_weather_api_key varchar(255) DEFAULT '';

COMMIT;
