INSERT INTO rooms(id, name, active, deactivate_after_timestamp)
VALUES
  (1, 'Alrum', true, NULL),
  (2, 'Ølrum', true, NULL),
  (3, 'Deactivated', false, NULL),
  (4, 'Expired', true, '2024-09-01');

INSERT INTO products(id, name, price, active, deactivate_after_timestamp)
VALUES 
  (1, 'Øl',               700 ,  true,  NULL),
  (2, 'Sodavand',         1200,  true,  NULL),
  (3, 'Søm',              200,   false, NULL),
  (4, 'Fytteturs Billet', 30000, true,  '2024-09-01');

INSERT INTO room_products(room_id, product_id)
VALUES
  (1, 1),
  (1, 2),
  (2, 1);

INSERT INTO product_aliases(alias_name, product_id)
VALUES
  ('øl',   1),
  ('soda', 2),
  ('cola', 2);

INSERT INTO users(id, username, email, notes)
VALUES
  (1, 'test_user', 'test@email.com', 'test user');

INSERT INTO deposits(amount, note, user_id)
VALUES
  (10000, 'test deposit', 1);

INSERT INTO news(id, content, active, deactivate_after_timestamp)
VALUES
  (1, 'This is a sample news item', true, NULL),
  (2, 'Another sample news item', true, NULL),
  (3, 'Deactivated news', false, NULL),
  (4, 'Expired', true, '2024-09-01');
