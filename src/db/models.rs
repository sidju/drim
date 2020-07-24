use super::*;
// Structs to represent database tables
// Unlikely to ever be used
#[derive(Debug)]
pub struct Role {
  pub name: String,
}
#[derive(Debug)]
pub struct User {
  pub id: i32,
  pub nick: String,
  pub email: String,
  pub pass: String,
  pub role: String,
}
#[derive(Debug)]
pub struct Session {
  pub id: i32,
  pub uuid: uuid::Uuid,
  pub userid: i32,
  pub until: DateTime,
}

// From here --------------
// All these structures need a file with the same name that has a get, a set and an update
// Look into users.rs for examples
pub struct Clans {
  pub name: String,
  pub url: String,
}
pub struct Sects {
  pub name: String,
  pub url: String,
}
// Character is also missing some field, write in based on below
pub struct Character {
  pub id: i32,
  pub userid: i32,
  // add stuff
}
/*
// Finally these tables need to be added to the migration script
// It resides in migrate.sql
// If you want you can give it a shot
CREATE TABLE Clans(
       Name VARCHAR PRIMARY KEY,
       URL VARCHAR
);

CREATE TABLE Sects(
       Name VARCHAR PRIMARY KEY,
       URL VARCHAR
);
CREATE TABLE Characters(
       id SERIAL PRIMARY KEY,
       userid INTEGER, --Soft link, so that players can be deleted--
       active BOOL NOT NULL,
       name VARCHAR(32) NOT NULL,
       sect VARCHAR NOT NULL,
       clan VARCHAR NOT NULL,
       date_embraced DATE NOT NULL,
       torpor_time INTERVAL NOT NULL,
       humanity INTEGER CHECK ( Humanity <= 10 AND Humanity >= 0 ) NOT NULL,

       FOREIGN KEY (sect) REFERENCES sects,
       FOREIGN KEY (clan) REFERENCES clans
);
*/
// To here ---------------




// Future problems
pub struct Influences {
  pub name: String,
  pub url: String,
}
pub struct Disciplines {
  pub name: String,
  pub url: String,
}
pub struct Cities {
  pub name: String,
}
pub struct Area {
  pub name: String,
  pub city: String
}

