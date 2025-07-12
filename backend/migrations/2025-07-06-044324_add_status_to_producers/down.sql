-- This file should undo anything in `up.sql`
ALTER TABLE producers DROP COLUMN status;
DROP TYPE producer_status_enum;