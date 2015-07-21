## connecting to mysql

mysql -u root -h localhost -p


create user 'test'@'localhost' identified by 'test';

create user 'bazaar'@'localhost' identified by 'b4z44r';

SET PASSWORD FOR 'test'@'localhost' = password('test');

 create database bazaar_v6;
 
 
 GRANT SELECT,INSERT,UPDATE,DELETE,CREATE,DROP on bazaar.* to 'bazaar'@'localhost';