#!/bin/sh

## create a new database, will prompt for password
psql -U postgres -h localhost -p 5432 -d postgres -c "create database bazaar_v8 with owner postgres encoding 'utf8';"

## create role bazaar
psql -U postgres -h localhost -p 5432 -d postgres -c "create role bazaar with login password 'b4z44r'"


## fill the newly created database, will prompt password again
psql -U postgres -h localhost -p 5432 -d bazaar_v8 -f ./scripts/bazaar_v8_all.sql


## change the password of postgresql as what is used in the examples and tests
psql -U postgres -h localhost -p 5432 -d postgres -c "alter role postgres with password 'p0stgr3s'"
