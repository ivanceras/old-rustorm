How to properly construct builder pattern in rust

https://aturon.github.io/ownership/builders.html


##Postgres specific foreign key
*This is needed because information_schema is a lot slower

http://stackoverflow.com/questions/1152260/postgres-sql-to-list-table-foreign-keys
```
SELECT
  o.conname AS constraint_name,
  (SELECT nspname FROM pg_namespace WHERE oid=m.relnamespace) AS source_schema,
  m.relname AS source_table,
  (SELECT a.attname FROM pg_attribute a WHERE a.attrelid = m.oid AND a.attnum = o.conkey[1] AND a.attisdropped = false) AS source_column,
  (SELECT nspname FROM pg_namespace WHERE oid=f.relnamespace) AS target_schema,
  f.relname AS target_table,
  (SELECT a.attname FROM pg_attribute a WHERE a.attrelid = f.oid AND a.attnum = o.confkey[1] AND a.attisdropped = false) AS target_column
FROM
  pg_constraint o LEFT JOIN pg_class c ON c.oid = o.conrelid
  LEFT JOIN pg_class f ON f.oid = o.confrelid LEFT JOIN pg_class m ON m.oid = o.conrelid
WHERE
  o.contype = 'f' AND o.conrelid IN (SELECT oid FROM pg_class c WHERE c.relkind = 'r');
 
```  
  
##Show tables in postgresql

select 
  tablename as table 
from 
  pg_tables  
where schemaname = 'public'


A simple Good resource for reading about lifetimes
http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html


##Select the parent table
```
select relname,  
( select relname from pg_class where oid = pg_inherits.inhparent ) as parent 
					 from pg_class  
					 left join pg_inherits  
					 on pg_class.oid = pg_inherits.inhrelid 
					 where relname = 'product'
```


##Select the subclass

```
select relname, 
( select relname from pg_class where oid = pg_inherits.inhrelid ) as subclass 
					 from pg_inherits 
					 left join pg_class on pg_class.oid = pg_inherits.inhparent 
					 where relname = 'base' ;
```


##Useful projects:

### text editor
https://github.com/gchp/iota

## csv parsing
https://github.com/BurntSushi/rust-csv

## window tiling for the win
https://github.com/Kintaro/wtftw

## for text base interface, a wrapper for termbox

https://github.com/gchp/rustbox


##How to deal with nullable columns in the database
*Most database columns has null values, can be optional
*If nullable make the field Optional


##Check to see if extension are already installed in the database
select * from pg_extension

You may need to create the schema before installing the extensions

