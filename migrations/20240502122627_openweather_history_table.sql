CREATE TABLE public.weather (
    id serial PRIMARY KEY,
    user_id integer REFERENCES public.users(id),
    date timestamp DEFAULT CURRENT_TIMESTAMP,
    temp varchar(255),
    feels_like varchar(255),
    humidity varchar(255),
    weather_main varchar(255),
    weather_description varchar(255),
    weather_icon varchar(255),
    city varchar(255),
    timezone real,
    counrty varchar(255)
);
