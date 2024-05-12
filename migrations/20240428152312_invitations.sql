CREATE TABLE invitations (
  id     INTEGER PRIMARY KEY NOT NULL,
  status TEXT NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Accepted', 'Rejected')),
  code   TEXT NOT NULL UNIQUE CHECK (length(code) = 4),
  email  TEXT NULL
);

CREATE TABLE guests
(
  id            INTEGER PRIMARY KEY NOT NULL,
  first_name    TEXT NOT NULL,
  last_name     TEXT NOT NULL,
  invitation_id INTEGER NOT NULL,
  FOREIGN KEY (invitation_id) REFERENCES invitations (id)
);
