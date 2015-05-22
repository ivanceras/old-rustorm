#!/bin/sh
pg_dump -U postgres -W -h localhost -p 5432  --schema-only bazaar_v4 > bazaar_v4_dump_schema.sql 
