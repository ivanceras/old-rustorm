#!/bin/sh

## create a new database, will prompt for password
psql -U postgres -h localhost -p 5432 -d postgres -c "create database bazaar_v6 with owner postgres encoding 'utf8';"

## fill the newly created database, will prompt password again
psql -U postgres -h localhost -p 5432 -d bazaar_v6 -f ./scripts/bazaar_v6_all.sql

psql -U postgres -h localhost -p 5432 -d postgres -c "alter role postgres with password 'p0stgr3s'"
