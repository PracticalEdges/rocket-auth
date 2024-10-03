-- Your SQL goes here

CREATE TABLE authorization_code (
  id CHAR(36) PRIMARY KEY,
  client_id CHAR(36) NOT NULL,
  user_id CHAR(36) NOT NULL,
  code TEXT NOT NULL,
  redirect_uri TEXT NOT NULL,
  expires_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES user(id),
  FOREIGN KEY (client_id) REFERENCES client(id)
);