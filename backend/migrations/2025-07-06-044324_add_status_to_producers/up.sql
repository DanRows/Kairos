-- Your SQL goes here
CREATE TYPE producer_status_enum AS ENUM ('pending', 'approved', 'rejected');
ALTER TABLE producers ADD COLUMN status producer_status_enum NOT NULL DEFAULT 'pending';