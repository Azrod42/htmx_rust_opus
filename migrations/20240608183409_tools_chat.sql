-- Add migration script here
CREATE TABLE public.tools_chat (
    id serial PRIMARY KEY,
    user_id integer REFERENCES public.users(id),
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    discusion varchar(255),
    prompt_tokens varchar(255),
    response_tokens varchar(255),
    total_tokens varchar(255)
);
