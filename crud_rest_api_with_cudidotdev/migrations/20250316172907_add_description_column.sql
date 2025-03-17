-- Add migration script here
ALTER TABLE tasks
ADD COLUMN IF NOT EXISTS description TEXT;