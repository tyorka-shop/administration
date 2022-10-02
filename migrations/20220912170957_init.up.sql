CREATE TABLE products (
  id VARCHAR(255) PRIMARY KEY NOT NULL,
  cover_id VARCHAR(255) NOT NULL,
  
  title_en TEXT NOT NULL,
  title_ru TEXT NOT NULL,

  show_in_gallery BOOLEAN NOT NULL DEFAULT FALSE,

  show_in_shop BOOLEAN NOT NULL DEFAULT FALSE,
  
  description_en TEXT NOT NULL,
  description_ru TEXT NOT NULL,
  
  price INTEGER,

  created_at datetime DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at datetime DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE `pictures` (
  id VARCHAR(255) PRIMARY KEY NOT NULL,
  color VARCHAR(255) NOT NULL,
  original_size_width INTEGER NOT NULL,
  original_size_height INTEGER NOT NULL,
  crop_anchor_x REAL NOT NULL,
  crop_anchor_y REAL NOT NULL,
  crop_factor REAL NOT NULL
);

CREATE TABLE blog (
  id VARCHAR(255) PRIMARY KEY NOT NULL,
  src VARCHAR(255) NOT NULL,
  url VARCHAR(255) NOT NULL,
  color VARCHAR(255) NOT NULL
);

CREATE TABLE entity_order (
  entity_id VARCHAR(255) NOT NULL,
  type VARCHAR(255) NOT NULL,
  idx INTEGER NOT NULL,
  PRIMARY KEY (entity_id, type)
);

CREATE TABLE `build` (
  `id` VARCHAR(255) PRIMARY KEY NOT NULL,
  `status` VARCHAR(255) NOT NULL,
  `log` TEXT NOT NULL,
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP NOT NULL,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE `product_pictures` (
  `product_id` VARCHAR(255) NOT NULL,
  `picture_id` VARCHAR(255) NOT NULL,
  `idx` INTEGER NOT NULL,
  PRIMARY KEY (product_id, picture_id)
);