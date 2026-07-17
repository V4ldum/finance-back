-- Null out coins.{obverse,reverse,edge} when they reference a coin_images row
-- that no longer exists. Foreign keys were not enforced at the connection level
-- before, so dangling references may exist. Going forward, PRAGMA foreign_keys
-- is enabled on every connection, which prevents new violations.

UPDATE coins SET obverse = NULL
    WHERE obverse IS NOT NULL
      AND obverse NOT IN (SELECT id FROM coin_images);

UPDATE coins SET reverse = NULL
    WHERE reverse IS NOT NULL
      AND reverse NOT IN (SELECT id FROM coin_images);

UPDATE coins SET edge = NULL
    WHERE edge IS NOT NULL
      AND edge NOT IN (SELECT id FROM coin_images);
