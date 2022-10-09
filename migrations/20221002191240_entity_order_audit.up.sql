CREATE TABLE `new_entity_order` (
  `entity_id` VARCHAR(255) NOT NULL,
  `type` VARCHAR(255) NOT NULL,
  `idx` INTEGER NOT NULL,
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP NOT NULL,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP NOT NULL,
  PRIMARY KEY (`entity_id`, `type`)
);

insert into `new_entity_order` (`entity_id`, `type`, `idx`) 
select `entity_id`, `type`, `idx` from `entity_order`;

drop table `entity_order`;
alter table `new_entity_order` rename to `entity_order`;
