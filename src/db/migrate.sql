CREATE OR REPLACE FUNCTION run_migrations() RETURNS BOOLEAN AS $$
  DECLARE
    migrations_exist BOOLEAN;
    migrations_version INTEGER;
  BEGIN
    -- First the initial migration if there is no migrations table
    SELECT INTO migrations_exist COALESCE((select FALSE from information_schema.tables where table_name = 'migrations'), TRUE);
    IF migrations_exist
    THEN
      -- If the migrations table doesn't exist no migrations can have been run
      -- thus we run the first migration
      -- Login entities--
      -- First the user roles, which permissions are based upon
      CREATE TABLE roles(
        name TEXT PRIMARY KEY
      );
      INSERT INTO roles VALUES ('user'),('gm'),('admin');
      -- Then the users, all the authentication essential data
      CREATE TABLE users(
        id SERIAL PRIMARY KEY,
        email TEXT NOT NULL UNIQUE CHECK ( email ~ '\S*@\S*\.\S*' ),
        pass TEXT,
        role TEXT,

        FOREIGN KEY (role) REFERENCES roles
      );
      -- Finally the session entries, UUID sent as auth cookie value
      CREATE TABLE sessions(
        id SERIAL PRIMARY KEY,
        userid INTEGER,
        uuid UUID UNIQUE,
        until DATE,

        FOREIGN KEY (userid) REFERENCES users
      );
      -- And mark that the migration has been run
      CREATE TABLE migrations(
        version INTEGER PRIMARY KEY
      );
      INSERT INTO migrations VALUES (1);
    END IF;
    -- Then the second if the max in migrations isn't equal or bigger than 2
    SELECT INTO migrations_version MAX(version) FROM migrations;
    RETURN TRUE;
  END;
$$ LANGUAGE plpgsql;

SELECT * FROM run_migrations();
