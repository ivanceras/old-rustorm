How to properly construct builder pattern in rust

https://aturon.github.io/ownership/builders.html




##Postgres specific foreign key
* This is needed because information_schema is a lot slower

http://stackoverflow.com/questions/1152260/postgres-sql-to-list-table-foreign-keys

```sql
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

```sql
select 
  tablename as table 
from 
  pg_tables  
where schemaname = 'public'
```


A simple Good resource for reading about lifetimes
http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html


##Select the parent table
```sql
select relname,  
( select relname from pg_class where oid = pg_inherits.inhparent ) as parent 
					 from pg_class  
					 left join pg_inherits  
					 on pg_class.oid = pg_inherits.inhrelid 
					 where relname = 'product'
```


##Select the subclass

```sql
select relname, 
( select relname from pg_class where oid = pg_inherits.inhrelid ) as subclass 
					 from pg_inherits 
					 left join pg_class on pg_class.oid = pg_inherits.inhparent 
					 where relname = 'base' ;
```


## Useful projects:

### text editor
https://github.com/gchp/iota

## csv parsing
https://github.com/BurntSushi/rust-csv

## window tiling for the win
https://github.com/Kintaro/wtftw

## for text base interface, a wrapper for termbox

https://github.com/gchp/rustbox


##How to deal with nullable columns in the database
* Most database columns has null values, can be optional
* If nullable make the field Optional


## Check to see if extension are already installed in the database
select * from pg_extension

You may need to create the schema before installing the extensions


## releasing:
```
cargo publish
```

Make sure license, github code, documentation is properly linked in the Cargo.toml file

## Publishing the documents

```
cargo clean
cargo doc --no-deps
cd target/doc
git init
git add . -A
git commit -m "Commiting docs to github pages"
git remote add origin https://github.com/ivanceras/rustorm
git checkout -b gh-pages
git push --force origin gh-pages


SHOW server_version;
select version();


## Uuid
* uuid has 16 bytes  -> (128 bits/8) 
* 32 characters (hex digit)
* 36 when including hyphens
* 22 character when encododed to base64

gotcha: need to put & to borrow immutable from mutable
```
fn execute_with_return(&mut self, query:&Query)->DaoResult{
        let sql_frag = &self.build_query(query);
```



### Get view columns
```sql

SELECT
                pg_attribute.attnum AS number,
                pg_attribute.attname AS name,
                pg_attribute.attnotnull AS notnull,
                pg_catalog.format_type(pg_attribute.atttypid, pg_attribute.atttypmod) AS data_type,
                CASE
                WHEN pg_constraint.contype = 'p' THEN true
                ELSE false
                END AS is_primary,
                CASE
                WHEN pg_constraint.contype = 'u' THEN true
                ELSE false
                END AS is_unique,
                CASE
                WHEN pg_constraint.contype = 'f' THEN g.relname
                END AS foreign_table,
                CASE
                WHEN pg_attribute.atthasdef = true THEN pg_attrdef.adsrc
                END as default
                ,pg_description.description as comment
                ,(SELECT nspname FROM pg_namespace WHERE oid=g.relnamespace) AS foreign_schema
                ,(SELECT pg_attribute.attname FROM pg_attribute
                WHERE pg_attribute.attrelid = pg_constraint.confrelid
                AND pg_attribute.attnum = pg_constraint.confkey[1]
                AND pg_attribute.attisdropped = false) AS foreign_column
                ,pg_constraint.conname

            FROM pg_attribute
                JOIN pg_class
                    ON pg_class.oid = pg_attribute.attrelid
                JOIN pg_type
                    ON pg_type.oid = pg_attribute.atttypid
                LEFT JOIN pg_attrdef
                    ON pg_attrdef.adrelid = pg_class.oid
                    AND pg_attrdef.adnum = pg_attribute.attnum
                LEFT JOIN pg_namespace
                    ON pg_namespace.oid = pg_class.relnamespace
                LEFT JOIN pg_constraint
                    ON pg_constraint.conrelid = pg_class.oid
                    AND pg_attribute.attnum = ANY (pg_constraint.conkey)
                LEFT JOIN pg_class AS g
                    ON pg_constraint.confrelid = g.oid
                LEFT JOIN pg_description
                    ON pg_description.objoid = pg_class.oid
                    AND pg_description.objsubid = pg_attribute.attnum
            WHERE pg_class.relkind = 'v'::char
                AND pg_namespace.nspname = 'views'
                AND pg_class.relname = 'vw_device_ssh'
                AND pg_attribute.attnum > 0
                ORDER BY number
```

## Sqlite meta data extractions
https://www.sqlite.org/pragma.html#pragma_foreign_key_list

### extract table columns
PRAGMA database.table_info(table-name);
PRAGMA table_info(product);

### foreign keys
PRAGMA foreign_key_list(table-name);
PRAGMA foreign_key_list(product_availability);

## indexes

```
PRAGMA index_list(table-name);


CREATE TABLE product_availability (
    product_id uuid NOT NULL,
    available boolean,
    always_available boolean,
    stocks numeric DEFAULT 1,
    available_from timestamp with time zone,
    available_until timestamp with time zone,
    available_day json,
    open_time time with time zone,
    close_time time with time zone,
    FOREIGN KEY(product_id) REFERENCES product(product_id)
)

## extract columns in sqlite, 
http://stackoverflow.com/questions/2785702/use-javascript-regex-to-extract-column-names-from-sqlite-create-table-sql

## Run test in the project using 

```
 cargo test --features "mysql sqlite"
```


https://en.wikipedia.org/wiki/Comparison_of_relational_database_management_systems


