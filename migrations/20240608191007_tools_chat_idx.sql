-- Add migration script here
ALTER TABLE public.tools_chat
ADD COLUMN idx varchar(255);
