#!/bin/sh
pg_dump -U postgres -W -h localhost -p 5432  bazaar_v4 > bazaar_v4_dump_all.sql 
