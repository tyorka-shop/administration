CREATE TABLE IF NOT EXISTS entity_order (
  entity_id VARCHAR(255) NOT NULL,
  type VARCHAR(255) NOT NULL,
  idx INTEGER NOT NULL,
  PRIMARY KEY (entity_id, type)
);