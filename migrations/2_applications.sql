CREATE TABLE applications (
    id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    action TEXT NOT NULL,

    PRIMARY KEY(id, name)
    FOREIGN KEY (id) REFERENCES locks(id)
);
