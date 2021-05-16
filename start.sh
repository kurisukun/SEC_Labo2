sqlite3 db.sqlite "CREATE TABLE users ( id integer not null primary key, email varchar not null, password varchar not null, two_factors_token varchar null, token_of_reset varchar null, token_date datetime null);"
sqlite3 db.sqlite "INSERT INTO users VALUES (1, 'chris.barroshenriques.heig@gmail.com', '1234', null, null, null);"
