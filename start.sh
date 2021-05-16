sqlite3 db.sqlite "CREATE TABLE person ( id int, name varchar(30), phone varchar(30) );"
sqlite3 db.sqlite "INSERT INTO person VALUES (1, 'Jim', '123446223');\
INSERT INTO person VALUES (2, 'Tom', '232124303');\
INSERT INTO person VALUES (3, 'Bill', '812947283');\
INSERT INTO person VALUES (4, 'Alice', '351246233');"
