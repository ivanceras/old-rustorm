## May 26, 2015
* Dump a sample database content to the bazaar

## June 9, 2015
* create an implementation fn from_dao(dao:Dao) for each model, this will be handy for converting records to rust objects
 
## June 12, 2015 
* Improve the implementation of table methods to 
get table references to have a unified logic

``

fn get_references()->RefTable
RefTable {  
    table,
    is_has_one,
    is_has_many,
    is_direct,
    is_ext,
}

impl RefTable{
    
    fn name(){
        //checks to avoid conflicting columns
        //checks to see if conflicts to other has_ones, has_many, ext
    }
}
``

## June 16, 2015
* Make the query api with filter work

## June 30, 2015
* Implement COPY from stdin 
http://sfackler.github.io/rust-postgres/doc/v0.9.2/postgres/struct.Statement.html#method.copy_in

* Add support for sqlite 
https://github.com/jgallagher/rusqlite

## July 1, 2015
* Support for deleting children on records that restrict deletion of referred records

## July 7, 2015
* Use r2d2 connection pooling
* use connection pooling for sqlite in memory database, such that only 1 instance of in-memory database with the same configuration will exist.

## July 17, 2015
* Add support for from_hashmap for DAO
* Add support for to_json for decoding the representation of the object when used as a primary object or an extension object
    * Extension object doesn't involve the both foreign and primary key values, null values etc.
    * Conver the object to hashmap, then remove the undesired column, then serialize fn serialize_compact()->String, fn concize_serialize()
    fn compact_hashmap()->HashMap;
    fn compact_json()->Json;
    to dao then serialize the json
    

## July 19, 2015

```rust

impl ProductAvailability{

    compact_json(&self)->Json{
        let mut dao = self.to_dao();
        dao.remove("product_id");
        dao.encode()
    }
}

```

## July 21, 2015
* Make compilation of underlying supported platform optional. Using the "feature" in Cargo.toml

## July 27, 2015

* Use const,
* add column module, list of all columns
* add table module list of all tables
* add schema modile list of all schema

## July 28, 2015

* Support for views
http://dba.stackexchange.com/questions/23836/how-to-list-all-views-in-sql-in-postgresql

```sql

select schemaname, viewname from pg_catalog.pg_views
where schemaname NOT IN ('pg_catalog', 'information_schema')
order by schemaname, viewname;

```

## September 9, 2015
* Replace rustc_serialize with serde-rs


## October 21, 2015
* Make mysql as optional dependency


## Macro

#[derive(IsDao,IsTable)]
pub struct Product{
    #[column(rename="product_name")]
    name: String,
    #[render(image_base64)]
    base64: String,
}




## April 12, 2016

* Refactor DAO to alias only to type Dao = BTreeMap<String, Value>
