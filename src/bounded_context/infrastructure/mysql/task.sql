USE rust;

CREATE TABLE task (
    id INT PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status ENUM('Todo', 'InProgress', 'Done') NOT NULL DEFAULT 'Todo'
);
