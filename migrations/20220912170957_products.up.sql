CREATE TABLE IF NOT EXISTS products (
  id VARCHAR(255) PRIMARY KEY NOT NULL,
  cover_id VARCHAR(255),
  
  title_en TEXT NOT NULL,
  title_ru TEXT NOT NULL,

  show_in_gallery BOOLEAN NOT NULL DEFAULT FALSE,

  show_in_shop BOOLEAN NOT NULL DEFAULT FALSE,
  
  description_en TEXT NOT NULL,
  description_ru TEXT NOT NULL,
  
  price INTEGER,
  
  'index' INTEGER
);