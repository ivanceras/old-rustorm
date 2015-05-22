#!/bin/sh
pg_dump -U postgres -W -h localhost -p 5432  --data-only bazaar_v4 > bazaar_v4_data.sql 
