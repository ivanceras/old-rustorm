#!/bin/sh
pg_dump -U postgres -h localhost -p 5432  --schema-only "$1" > "$1_dump_schema.sql" 
