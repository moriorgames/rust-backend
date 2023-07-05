USE rust;

CREATE TABLE task (
    id VARCHAR(36) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    status ENUM('Todo', 'InProgress', 'Done') NOT NULL DEFAULT 'Todo'
);
