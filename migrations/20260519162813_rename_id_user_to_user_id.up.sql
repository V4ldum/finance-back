-- Rename `id_user` to `user_id` in cash_assets and raw_assets for consistency
-- with coin_assets. SQLite supports RENAME COLUMN since 3.25 and automatically
-- updates the column name in FK definitions.

ALTER TABLE cash_assets RENAME COLUMN id_user TO user_id;
ALTER TABLE raw_assets  RENAME COLUMN id_user TO user_id;
