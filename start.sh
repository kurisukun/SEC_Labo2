sqlite3 db.sqlite "CREATE TABLE users ( id integer not null primary key, email varchar not null, password varchar not null, two_factors boolean not null);"
sqlite3 db.sqlite "INSERT INTO users VALUES (1, 'chris.barroshenriques.heig@gmail.com', '1234', false);"
