use query::Query;
use table::{Table, Column, Foreign};
use dao::Dao;

use postgres::Connection;
use regex::Regex;
use dao::Value;
use database::{Database, DatabaseDev, DatabaseDDL, DbError};
use postgres::types::Type as PgType;
use postgres::types::ToSql;
use writer::SqlFrag;
use postgres::rows::Row;
use database::SqlOption;
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;
use rustc_serialize::json::Json;
use dao::Type;
use postgres::types::IsNull;
use uuid::Uuid;
use query::Operand;

pub struct Postgres {
    /// a connection pool is provided
    pub pool: Option<PooledConnection<PostgresConnectionManager>>,
}

/// Build the Query into a SQL statements that is a valid
/// PostgreSQL sql query,
/// TODO: support version SqlOptions/specific syntax

//static none: &'static Option<String> = &None;

impl Postgres {

    /// create an instance, but without a connection yet,
    /// useful when just building sql queries specific to this platform
    /// inexpensive operation, so can have multiple instances
    pub fn new() -> Self {
        Postgres { pool: None }
    }


    pub fn with_pooled_connection(pool: PooledConnection<PostgresConnectionManager>) -> Self {
        Postgres { pool: Some(pool) }
    }



    pub fn get_connection(&self) -> &Connection {
        match self.pool {
            Some(ref pool) => &pool,
            None => panic!("No connection for this database"),
        }
    }

    /// convert Type to ToSql (postgresql native types)
    /// This is used when inserting records to the database
    /// TODO: put this somewhere organized
    /// TODO: match all the other filter types
    /// TODO: need to have a container for PgType contained before being borrowed to actual postgres type
    fn from_rust_type_tosql<'b>(&self, types: &'b [Value]) -> Vec<&'b ToSql> {
        let mut params: Vec<&ToSql> = vec![];
        for t in types {
            match *t {
                Value::Bool(ref x) => params.push(x),
                Value::I8(ref x) => params.push(x),
                Value::I16(ref x) => params.push(x),
                Value::I32(ref x) => params.push(x),
                Value::I64(ref x) => params.push(x),
                Value::U8(_) => panic!("unsupported/unexpected type! {:?}", t),
                Value::U16(_) => panic!("unsupported/unexpected type! {:?}", t),
                Value::U32(ref x) => params.push(x),
                Value::U64(_) => panic!("unsupported/unexpected type! {:?}", t),
                Value::F32(ref x) => params.push(x),
                Value::F64(ref x) => params.push(x),
                Value::String(ref x) => params.push(x),
                Value::VecU8(ref x) => params.push(x),
                Value::Uuid(ref x) => params.push(x),
                Value::DateTime(ref x) => params.push(x),
                Value::NaiveDate(ref x) => params.push(x),
                Value::NaiveTime(ref x) => params.push(x),
                Value::NaiveDateTime(ref x) => params.push(x),
                Value::Json(ref x) => {
//                    panic!("Json is not yet supported!..");
//                     static NONE: &'static Option<String> = &None;
                       params.push(x)
                }
                Value::None(ref v_type) => {
                        match v_type{
							&Type::String => {
								static none: &'static Option<String> = &None;
								params.push(none)
							},
							&Type::Uuid => {
								static none: &'static Option<Uuid> = &None;
								params.push(none)
							}
							_ => panic!("not yet for Non type of {:?}",v_type),
						}
                    },
                _ => panic!("not yet here {:?}", t),
            }
        }
        params
    }
    

    /// convert a record of a row into rust type
    fn from_sql_to_rust_type(&self, dtype: &PgType, row: &Row, index: usize) -> Value {
        match *dtype {
            PgType::Uuid => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::Uuid(value),
                    Err(_) => Value::None(Type::Uuid),
                }
            }
            PgType::Varchar | PgType::Text | PgType::Bpchar => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::String(value),
                    Err(_) => Value::None(Type::String),
                }
            }
            PgType::TimestampTZ | PgType::Timestamp => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::DateTime(value),
                    Err(_) => Value::None(Type::DateTime),
                }
            }
            PgType::Float4 => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::F32(value),
                    Err(_) => Value::None(Type::F32),
                }
            }
            PgType::Float8 => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::F64(value),
                    Err(_) => Value::None(Type::F64),
                }
            }
            PgType::Numeric => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::F64(value),
                    Err(_) => Value::None(Type::F64),
                }
            }, 
            PgType::Bool => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::Bool(value),
                    Err(_) => Value::None(Type::F64),
                }
            }
            PgType::Json => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::Json(value),
                    Err(_) => Value::None(Type::F64),
                }
            }
            PgType::Int2 => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::I16(value),
                    Err(_) => Value::None(Type::F64),
                }
            }
            PgType::Int4 => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::I32(value),
                    Err(_) => Value::None(Type::I32),
                }
            }
            PgType::Int8 => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::I64(value),
                    Err(_) => Value::None(Type::I64),
                }
            }
            PgType::Timetz => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::DateTime(value),
                    Err(_) => Value::None(Type::DateTime),
                }
            }
            PgType::Date => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::DateTime(value),
                    Err(_) => Value::None(Type::DateTime),
                }
            }
            PgType::Bytea => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::VecU8(value),
                    Err(_) => Value::None(Type::VecU8),
                }
            }
            PgType::Inet => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::String(value),
                    Err(_) => Value::None(Type::String),
                }
            }
            PgType::Tsvector => {
                let value = row.get_opt(index);
                match value {
                    Ok(value) => Value::String(value),
                    Err(_) => Value::None(Type::String),
                }
            }
            _ => panic!("Type {:?} is not covered!", dtype),
        }
    }


    ///
    /// http://stackoverflow.com/questions/109325/postgresql-describe-table
    ///
    fn get_table_columns(&self, schema: &str, table: &str) -> Vec<Column> {
        let sql = "
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
            WHERE pg_class.relkind IN ('r','v')
                AND pg_namespace.nspname = $1
                AND pg_class.relname = $2
                AND pg_attribute.attnum > 0
                ORDER BY number
            ";
        let conn = self.get_connection();
        let stmt = conn.prepare(&sql).unwrap();
        let mut columns = Vec::new();
        for row in stmt.query(&[&schema, &table]).unwrap() {
            let name: String = row.get("name");
            let not_null: bool = row.get("notnull");
            let db_data_type: String = row.get("data_type");
            //TODO: temporarily regex the data type to extract the size as well
            let re = match Regex::new("(.+)\\((.+)\\)") {
                Ok(re) => re,
                Err(err) => panic!("{}", err),
            };

            let db_data_type = if re.is_match(&db_data_type) {
                let cap = re.captures(&db_data_type).unwrap();
                let data_type = cap.at(1).unwrap().to_owned();
                // TODO::can be use in the later future
                // let size = cap.at(2).unwrap().to_owned();
                data_type
            } else {
                db_data_type
            };

            let is_primary: bool = row.get("is_primary");
            let is_unique: bool = row.get("is_unique");

            let default: Option<Operand> = match row.get_opt("default") {
                Ok(x) => Some(Operand::Value(Value::String(x))),
                Err(_) => None,
            };
            let comment: Option<String> = match row.get_opt("comment") {
                Ok(x) => Some(x),
                Err(_) => None,
            };

            let foreign_schema: Option<String> = match row.get_opt("foreign_schema") {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let foreign_column: Option<String> = match row.get_opt("foreign_column") {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let foreign_table: Option<String> = match row.get_opt("foreign_table") {
                Ok(x) => Some(x),
                Err(_) => None,
            };


            let foreign = if foreign_table.is_some() && foreign_column.is_some() &&
                             foreign_schema.is_some() {
                Some(Foreign {
                    schema: foreign_schema,
                    table: foreign_table.unwrap(),
                    column: foreign_column.unwrap(),
                })

            } else {
                None
            };
            let (_, data_type) = self.dbtype_to_rust_type(&db_data_type);
            let column = Column {
				table: Some(table.to_owned()),
                name: name,
                data_type: data_type,
                db_data_type: db_data_type,
                comment: comment,
                is_primary: is_primary,
                is_unique: is_unique,
                default: default,
                not_null: not_null,
                foreign: foreign,
                is_inherited: false, /* will be corrected later in the get_meta_data */
            };
            columns.push(column);
        }
        //unify due to the fact that postgresql return a separate row for
        // both primary and foreign columns
        self.unify_primary_and_foreign_column(&columns)
    }

    fn get_table_comment(&self, schema: &str, table: &str) -> Option<String> {
        let sql = "
                SELECT
                    pg_class.relname AS table,
                    pg_namespace.nspname AS schema,
                    obj_description(pg_class.oid) AS comment
                FROM pg_class
                    LEFT JOIN pg_namespace
                        ON pg_namespace.oid = pg_class.relnamespace
                WHERE
                    pg_class.relkind IN ('r','v')
                    AND pg_namespace.nspname NOT IN ('information_schema', 'pg_catalog', 'pg_toast')
                    AND nspname = $1
                    AND relname = $2
                ";
        let conn = self.get_connection();
        let stmt = conn.prepare(&sql).unwrap();
        for row in stmt.query(&[&schema, &table]).unwrap() {
            let comment: Option<String> = match row.get_opt("comment") {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            return comment;
        }
        None
    }

    /// column that is both primary and foreign should be unified
    fn unify_primary_and_foreign_column(&self, columns: &[Column]) -> Vec<Column> {
        let mut unified_columns = Vec::new();
        let mut primary_columns = Vec::new();
        let mut foreign_columns = Vec::new();
        for c in columns {
            if c.is_primary {
                primary_columns.push(c.name.clone());
            }
            if c.foreign.is_some() {
                foreign_columns.push(c.name.clone());
            }
        }
        //if both primary and foreign, push only the modified foreign
        for c in columns {
            if primary_columns.contains(&c.name) && foreign_columns.contains(&c.name) {
                if c.foreign.is_some() {
                    let mut clone_column = c.clone();
                    clone_column.is_primary = true;
                    unified_columns.push(clone_column);
                }
            } else {
                unified_columns.push(c.clone());
            }
        }
        unified_columns

    }

}


impl Database for Postgres {
    fn version(&self) -> Result<String, DbError> {
        let sql = "SHOW server_version";
        let dao = try!(self.execute_sql_with_one_return(sql, &vec![]));
        match dao {
            Some(dao) => Ok(dao.get("server_version")),
            None => Err(DbError::new("Unable to get database version")),
        }
    }

    fn begin(&self) {
    }
    fn commit(&self) {
    }
    fn rollback(&self) {
    }
    fn is_transacted(&self) -> bool {
        false
    }
    fn is_closed(&self) -> bool {
        false
    }
    fn is_connected(&self) -> bool {
        false
    }
    fn close(&self) {
    }
    fn is_valid(&self) -> bool {
        false
    }
    fn reset(&self) {
    }

    /// return this list of options, supported features in the database
    /// TODO: make this features version specific
    /// http://www.postgresql.org/about/featurematrix/
    /// writer CTE  >= 9.1
    /// Inheritance  >= 9.0
    /// JSON >= 9.2
    /// JSONB >= 9.4
    /// Returning >= 8.2
    fn sql_options(&self) -> Vec<SqlOption> {
        vec![
            SqlOption::UsesNumberedParam,  // uses numbered parameters
            SqlOption::SupportsReturningClause, // supports returning clause, feature
            SqlOption::SupportsCTE,
            SqlOption::SupportsInheritance,
            SqlOption::UsesSchema,
            SqlOption::ReturnMetaColumns,// whether to use the column names returned in a statement
        ]
    }


    fn update(&self, _query: &Query) -> Dao {
        unimplemented!()
    }
    fn delete(&self, _query: &Query) -> Result<usize, String> {
        unimplemented!()
    }

    fn execute_sql_with_return(&self, sql: &str, params: &[Value]) -> Result<Vec<Dao>, DbError> {
        debug!("SQL: \n{}", sql);
        debug!("param: {:?}", params);
        let conn = self.get_connection();
        let stmt = try!(conn.prepare(sql));
        let mut daos = vec![];
        let param = self.from_rust_type_tosql(params);
        let rows = try!(stmt.query(&param));
        for row in rows {
            let columns = row.columns();
            let mut index = 0;
            let mut dao = Dao::new();
            for c in columns {
                let column_name = c.name();
                let dtype = c.type_();
                let rtype = self.from_sql_to_rust_type(&dtype, &row, index);
                dao.set_value(column_name, rtype);
                index += 1;
            }
            daos.push(dao);
        }
        Ok(daos)
    }

    /// generic execute sql which returns not much information,
    /// returns only the number of affected records or errors
    /// can be used with DDL operations (CREATE, DELETE, ALTER, DROP)
    fn execute_sql(&self, sql: &str, params: &[Value]) -> Result<usize, DbError> {
        debug!("SQL: \n{}", sql);
        debug!("param: {:?}", params);
        let to_sql_types = self.from_rust_type_tosql(params);
        let conn = self.get_connection();
        let result = try!(conn.execute(sql, &to_sql_types));
        Ok(result as usize)
    }

}

impl DatabaseDDL for Postgres {

    fn create_schema(&self, _schema: &str) {
        unimplemented!()
    }
    fn drop_schema(&self, _schema: &str) {
        unimplemented!()
    }
    fn create_table(&self, _model: &Table) {
        unimplemented!()
    }
    fn build_create_table(&self, _table: &Table) -> SqlFrag {
        unimplemented!()
    }
    fn rename_table(&self, _table: &Table, _new_tablename: String) {
        unimplemented!()
    }
    fn drop_table(&self, _table: &Table) {
        unimplemented!()
    }
    fn set_foreign_constraint(&self, _model: &Table) {
        unimplemented!()
    }
    fn set_primary_constraint(&self, _model: &Table) {
        unimplemented!()
    }

}

/// this can be condensed with using just extracting the table definition
impl DatabaseDev for Postgres {

    fn get_parent_table(&self, schema: &str, table: &str) -> Option<String> {
        let sql = "
            SELECT
                relname as table,
                pg_namespace.nspname as schema,
                ( SELECT relname FROM pg_class WHERE oid = pg_inherits.inhparent ) AS parent_table
             FROM pg_class
             INNER JOIN pg_namespace
                ON pg_class.relnamespace = pg_namespace.oid
             LEFT JOIN pg_inherits
                 ON pg_class.oid = pg_inherits.inhrelid
             WHERE pg_namespace.nspname = $1
                 AND relname = $2
                ";
        let conn = self.get_connection();
        let stmt = conn.prepare(&sql).unwrap();
        for row in stmt.query(&[&schema, &table]).unwrap() {
            let parent_table: Option<String> = match row.get_opt("parent_table") {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            return parent_table;
        }
        None
    }

    fn get_table_sub_class(&self, schema: &str, table: &str) -> Vec<String> {
        let sql = "
            SELECT
                relname AS base_table,
                ( SELECT relname FROM pg_class WHERE oid = pg_inherits.inhrelid ) AS sub_class
             FROM pg_inherits
             LEFT JOIN pg_class
                ON pg_class.oid = pg_inherits.inhparent
             INNER JOIN pg_namespace
                ON pg_class.relnamespace = pg_namespace.oid
             WHERE pg_namespace.nspname = $1
             AND relname = $2
             ORDER BY relname
            ";
        let conn = self.get_connection();
        let stmt = conn.prepare(&sql).unwrap();
        let mut sub_classes: Vec<String> = vec![];
        for row in stmt.query(&[&schema, &table]).unwrap() {
            match row.get_opt("sub_class") {
                Ok(x) => sub_classes.push(x),
                Err(_) => (),
            }
        }
        sub_classes
    }



    fn get_table_metadata(&self, schema: &str, table: &str, is_view: bool) -> Table {

        let mut columns = self.get_table_columns(schema, table);
        let comment = self.get_table_comment(schema, table);
        let parent = self.get_parent_table(schema, table);
        let subclass = self.get_table_sub_class(schema, table);

        //mutate columns to mark those which are inherited
        if parent.is_some() {
            let inherited_columns = self.get_inherited_columns(schema, table);
            for i in inherited_columns {
                for c in &mut columns {
                    if i == c.name {
                        c.is_inherited = true;
                    }
                }
            }
        }

        Table {
            schema: Some(schema.to_owned()),
            name: table.to_owned(),
            parent_table: parent,
            sub_table: subclass,
            comment: comment,
            columns: columns,
            is_view: is_view,
        }
    }

    fn get_all_tables(&self) -> Vec<(String, String, bool)> {
        let sql = "
                SELECT
                    pg_class.relname AS table,
                    pg_namespace.nspname AS schema,
                    obj_description(pg_class.oid) AS comment,
                    CASE
                        WHEN pg_class.relkind = 'r' THEN false
                        WHEN pg_class.relkind = 'v' THEN true
                    END AS is_view
                FROM pg_class
                    LEFT JOIN pg_namespace
                        ON pg_namespace.oid = pg_class.relnamespace
                WHERE
                    pg_class.relkind IN ('r','v')
                    AND pg_namespace.nspname NOT IN ('information_schema', 'pg_catalog', 'pg_toast')
                ORDER BY relname, nspname

                ";
        let conn = self.get_connection();
        let stmt = conn.prepare(&sql).unwrap();
        let mut tables: Vec<(String, String, bool)> = Vec::new();
        for row in stmt.query(&[]).unwrap() {
            let table: String = row.get("table");
            let schema: String = row.get("schema");
            let is_view: bool = row.get("is_view");
            tables.push((schema, table, is_view));
        }
        tables
    }



    fn get_inherited_columns(&self, schema: &str, table: &str) -> Vec<String> {
        let sql = "
                SELECT nmsp_parent.nspname    AS parent_schema,
                    parent.relname         AS parent_table,
                    nmsp_child.nspname     AS child_schema,
                       child.relname          AS child_table,
                       column_parent.attname  AS column_parent_name
                FROM pg_inherits
                    JOIN pg_class parent
                        ON pg_inherits.inhparent  = parent.oid
                    JOIN pg_class child
                        ON pg_inherits.inhrelid   = child.oid
                    JOIN pg_namespace nmsp_parent
                        ON nmsp_parent.oid        = parent.relnamespace
                    JOIN pg_namespace nmsp_child
                        ON nmsp_child.oid         = child.relnamespace
                    JOIN pg_attribute column_parent
                        ON column_parent.attrelid = parent.oid
                    WHERE column_parent.attnum > 0
                    AND nmsp_child.nspname = $1
                    AND child.relname = $2
                    ORDER BY column_parent.attname
                ";
        let conn = self.get_connection();
        let stmt = conn.prepare(&sql).unwrap();
        let mut inherited_columns = Vec::new();
        for row in stmt.query(&[&schema, &table]).unwrap() {
            let column: String = row.get("column_parent_name");
            inherited_columns.push(column);
        }
        inherited_columns
    }


    /// get the rust data type names from database data type names
    /// will be used in source code generation
    fn dbtype_to_rust_type(&self, db_type: &str) -> (Vec<String>, Type) {
        match db_type {
            "boolean" => {
                (vec![], Type::Bool)
            }
            "char" => {
                (vec![], Type::I8)
            }
            "smallint" | "smallserial" => {
                (vec![], Type::I16)
            }
            "integer" | "int" | "serial" => {
                (vec![], Type::I32)
            }
            "oid" => {
                (vec![], Type::U32)
            }
            "bigint" | "bigserial" => {
                (vec![], Type::I64)
            }
            "real" => {
                (vec![], Type::F32)
            }
            "numeric" => {
                (vec![], Type::F64)
                //panic!("No support for numeric yet, please use real or double precision")
            }
            "double precision" => {
                (vec![], Type::F64)
            }
            "name" | "character" | "character varying" | "text" | "citext" | "bpchar" => {
                (vec![], Type::String)
            }
            "bytea" => {
                (vec![], Type::VecU8)
            }
            "json" | "jsonb" => {
                (vec!["rustc_serialize::json::Json".to_owned()], Type::Json)
            }
            "uuid" => {
                (vec!["uuid::Uuid".to_owned()], Type::Uuid)
            }
            "timestamp" => {
                (vec!["chrono::naive::datetime::NaiveDateTime".to_owned()],
                 Type::NaiveDateTime)
            }
            "timestamp without time zone" => {
                (vec!["chrono::naive::datetime::NaiveDateTime".to_owned()],
                 Type::NaiveDateTime)
            }
            "timestamp with time zone" => {
                (vec!["chrono::datetime::DateTime".to_owned(),
                      "chrono::offset::utc::UTC".to_owned()],
                 Type::DateTime)
            }
            "time with time zone" => {
                (vec!["chrono::naive::time::NaiveTime".to_owned(),
                      "chrono::offset::utc::UTC".to_owned()],
                 Type::NaiveTime)
            }
            "date" => {
                (vec!["chrono::naive::date::NaiveDate".to_owned()],
                 Type::NaiveDate)
            }
            "time" => {
                (vec!["chrono::naive::time::NaiveTime".to_owned()],
                 Type::NaiveTime)
            }
            "hstore" => {
                (vec!["std::collections::HashMap".to_owned()],
                 Type::Object)
            }
            "interval" => {
                (vec![], Type::U32)
            }
            "inet[]" => {
                (vec![], Type::String)
            }
            "tsvector" | "inet" => {
                (vec![], Type::String)
            }//or everything else should be string
            _ => panic!("Unable to get the equivalent data type for {}", db_type),
        }
    }

    ///
    /// convert rust data type names to database data type names
    /// will be used in generating SQL for table creation
    /// FIXME, need to restore the exact data type as before
    fn rust_type_to_dbtype(&self, rust_type: &Type) -> String {
        match *rust_type {
            Type::Bool => {
                "boolean".to_owned()
            }
            Type::I8 => {
                "char".to_owned()
            }
            Type::I16 => {
                "smallint".to_owned()
            }
            Type::I32 => {
                "integer".to_owned()
            }
            Type::U32 => {
                "oid".to_owned()
            }
            Type::I64 => {
                "bigint".to_owned()
            }
            Type::F32 => {
                "real".to_owned()
            }
            Type::F64 => {
                "double precision".to_owned()
            }
            Type::String => {
                "character varying".to_owned()
            }
            Type::VecU8 => {
                "bytea".to_owned()
            }
            Type::Json => {
                "json".to_owned()
            }
            Type::Uuid => {
                "uuid".to_owned()
            }
            Type::NaiveDateTime => {
                "timestamp".to_owned()
            }
            Type::DateTime => {
                "timestamp with time zone".to_owned()
            }
            Type::NaiveDate => {
                "date".to_owned()
            }
            Type::NaiveTime => {
                "time".to_owned()
            }
            Type::Object => {
                "hstore".to_owned()
            }
            _ => panic!("Unable to get the equivalent database data type for {:?}",
                        rust_type),
        }
    }

}
