CREATE TABLE `new_product_pictures` (
  `product_id` VARCHAR(255) NOT NULL,
  `picture_id` VARCHAR(255) NOT NULL,
  `idx` INTEGER NOT NULL,
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP NOT NULL,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP NOT NULL,
  PRIMARY KEY (product_id, picture_id)
);

INSERT into `new_product_pictures` (`product_id`, `picture_id`, `idx`)
select `product_id`, `picture_id`, `idx` from `product_pictures`;

drop table `product_pictures`;
alter TABLE `new_product_pictures` rename to `product_pictures`;