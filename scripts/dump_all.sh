#!/bin/sh
pg_dump -U postgres -h localhost -p 5432  "$1" > "$1_all.sql" 
