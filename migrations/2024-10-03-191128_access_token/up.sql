-- Your SQL goes here

CREATE TABLE access_token (
    id CHAR(36) PRIMARY KEY,
    client_id CHAR(36) NOT NULL,
    user_id CHAR(36) NOT NULL,
    token TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES user(id),
    FOREIGN KEY (client_id) REFERENCES client(id)
);