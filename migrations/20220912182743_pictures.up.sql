CREATE TABLE IF NOT EXISTS `pictures` (
  id VARCHAR(255) PRIMARY KEY NOT NULL,
  color VARCHAR(255) NOT NULL,
  original_size_width INTEGER NOT NULL,
  original_size_height INTEGER NOT NULL,
  crop_anchor_x REAL NOT NULL,
  crop_anchor_y REAL NOT NULL,
  crop_factor REAL NOT NULL,
  product_id VARCHAR(255) 
);