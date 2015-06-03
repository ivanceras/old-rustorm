use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;


///
/// Base table contains the creation and modification status of a record
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Base {
    /// db data type: uuid
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// db data type: timestamp with time zone
    pub created:DateTime<UTC>,
    /// db data type: uuid
    pub createdby:Option<Uuid>,
    /// db data type: uuid
    pub organization_id:Option<Uuid>,
    /// priority of saving data and eviction
    /// db data type: numeric
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// db data type: timestamp with time zone
    pub updated:DateTime<UTC>,
    /// db data type: uuid
    pub updatedby:Option<Uuid>,
}


///
/// All User table should inherit from this one
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Record {
    /// @Active
    /// default: true
    /// not nullable 
    /// db data type: boolean
    pub active:bool,
    /// db data type: character varying
    pub description:Option<String>,
    /// db data type: text
    pub help:Option<String>,
    /// db data type: character varying
    pub name:Option<String>,
    /// --inherited-- 
    /// db data type: uuid
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    /// db data type: timestamp with time zone
    pub created:DateTime<UTC>,
    /// --inherited-- 
    /// db data type: uuid
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    /// db data type: uuid
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    /// db data type: numeric
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    /// db data type: timestamp with time zone
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    /// db data type: uuid
    pub updatedby:Option<Uuid>,
}
