-- Add up migration script here

CREATE TABLE IF NOT EXISTS company (
    org_nr INTEGER PRIMARY KEY,
    name VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS forvalter (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    org_nr INTEGER,
    name VARCHAR(255),
    added_at TIMESTAMP WITH TIME ZONE,
    FOREIGN KEY(org_nr) REFERENCES COMPANY(org_nr)
);

-- INSERT INTO COMPANY (org_nr, name) VALUES (983266908, "Verdipapirfondet DnB Teknologi");

-- INSERT INTO FORVALTER (org_nr, name, added_at) VALUES (983266908, "Anders Tandberg-Johansen", "2025-02-28 12:49:27");
-- INSERT INTO FORVALTER (org_nr, name, added_at) VALUES (983266908, "Sverre Bergland", "2025-02-28 12:49:27");
-- INSERT INTO FORVALTER (org_nr, name, added_at) VALUES (983266908, "Erling Thune", "2025-02-28 12:49:27");
-- INSERT INTO FORVALTER (org_nr, name, added_at) VALUES (983266908, "Erling Haugan Kise", "2025-02-28 12:49:27");