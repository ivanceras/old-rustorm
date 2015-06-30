## May 26, 2015
* Dump a sample database content to the bazaar

## June 9, 2015
* create an implementation fn from_dao(dao:Dao) for each model, this will be handy for converting records to rust objects
 
## June 12, 2015 
* Improve the implementation of table methods to 
get table references to have a unified logic
 get_references()->RefTable
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

## June 16, 2015
* Make the query api with filter work

## June 30, 2015
* Implement COPY from stdin 
http://sfackler.github.io/rust-postgres/doc/v0.9.2/postgres/struct.Statement.html#method.copy_in

* Add support for sqlite 
https://github.com/jgallagher/rusqlite

## July 1, 2015
* Support for deleting children on records that restrict deletion of referred records
