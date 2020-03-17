--Login entities--
--Possible roles--
CREATE TABLE roles(
  name TEXT PRIMARY KEY
);

INSERT INTO roles VALUES ('user'),('gm'),('admin');

--Auth-essential user data--
CREATE TABLE users(
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL UNIQUE CHECK ( eMail ~ '\S*@\S*\.\S*' ),
  pass TEXT, --Salted and hashed using argon--
  role TEXT,

  FOREIGN KEY (role) REFERENCES roles
);

--Session data, UUID randomly generated and sent as auth cookie--
CREATE TABLE sessions(
  id SERIAL PRIMARY KEY,
  userid INTEGER,
  uuid UUID UNIQUE,
  until DATE,

  FOREIGN KEY (userid) REFERENCES users
);

--Strictly relevant off--
CREATE TABLE players(
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  nationalid TEXT CHECK ( nationalid ~ '\d{10}' ),
  email TEXT NOT NULL UNIQUE CHECK ( email ~ '\S*@\S*\.\S*' ),
  telephone TEXT NOT NULL CHECK ( telephone ~ '\+\d{1,4}|0\d{8,9}'),
  active BOOL NOT NULL,
  paid BOOL NOT NULL,
  inebas BOOL NOT NULL
);
