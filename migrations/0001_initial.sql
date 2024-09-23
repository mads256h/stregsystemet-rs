CREATE TABLE users (
  id SERIAL PRIMARY KEY NOT NULL,
  username VARCHAR(128) NOT NULL CONSTRAINT no_whitespace CHECK(LENGTH(username) != 0 AND username NOT LIKE '% %'),
  email VARCHAR(128) NOT NULL,
  notes VARCHAR NOT NULL,
  join_timestamp TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX users_username_key ON users(LOWER(username));
CREATE UNIQUE INDEX users_email_key ON users(LOWER(email));

CREATE TABLE products (
  id SERIAL PRIMARY KEY NOT NULL,
  name VARCHAR(128) NOT NULL,
  price BIGINT NOT NULL CONSTRAINT nonnegative_price CHECK(price >= 0),
  active BOOLEAN NOT NULL,
  deactivate_after_timestamp TIMESTAMPTZ
);

CREATE TABLE product_aliases (
  alias_name VARCHAR(128) PRIMARY KEY NOT NULL CONSTRAINT lower_case_and_no_whitespace_or_colon CHECK(LENGTH(alias_name) != 0 AND alias_name NOT LIKE '% %' AND alias_name NOT LIKE '%:%' AND alias_name = LOWER(alias_name)),
  product_id SERIAL NOT NULL,

  CONSTRAINT fk_product 
    FOREIGN KEY(product_id)
      REFERENCES products(id)
        ON DELETE CASCADE
);

CREATE TABLE sales (
  id SERIAL PRIMARY KEY NOT NULL,
  price BIGINT NOT NULL CONSTRAINT nonnegative_price CHECK(price >= 0),
  timestamp TIMESTAMPTZ NOT NULL DEFAULT now(),
  product_id SERIAL NOT NULL,
  user_id SERIAL NOT NULL,
  
  CONSTRAINT fk_product
    FOREIGN KEY(product_id)
      REFERENCES products(id),

  CONSTRAINT fk_user
    FOREIGN KEY(user_id)
      REFERENCES users(id)
);

CREATE TABLE deposits (
  id SERIAL PRIMARY KEY NOT NULL,
  amount BIGINT NOT NULL CONSTRAINT positive_amount CHECK(amount > 0),
  timestamp TIMESTAMPTZ NOT NULL DEFAULT now(),
  note VARCHAR NOT NULL,
  user_id SERIAL NOT NULL,

  CONSTRAINT fk_user
    FOREIGN KEY(user_id)
      REFERENCES users(id)
);
