//! Provides database pooling


use platform::Platform;
use config::DbConfig;
use platform::Sqlite;
use platform::Postgres;


/// This pool contains database that are not necessarily same platform and configs
/// database pools are stored using treemap, with the key is the url of the database
/// owns the database, and only lend out reference when api tries to connect to the database
/// TODO: protect the free form here with locks
pub struct Pool{
    /// the available list of connections
    free: Vec<Platform>,
    /// the number of used count, may not be accurate
    used: usize,
}


impl Pool{
    
    /// initialize a pool for database connection container
    /// call only once per application
    pub fn init()->Self{
        Pool{free: vec![], used: 0}
    }
    
    
    pub fn reserve(url:&str, n: usize)->Self{
        let mut pool = Self::init();
        pool.reserve_connection(url, n);
        pool
    }
    /// reserve a number of connections using the url config
    pub fn reserve_connection(&mut self, url:&str, n: usize)->&mut Self{
        let db_config = DbConfig::from_url(url);
        for _ in 0..n{
            match self.add_connection(&db_config){
                Ok(_) => (),
                Err(_) => panic!("can not add more connection"),
            }
        }
        self
    }
    
    /// create a new database connection,
    /// and add it to the free pool
    /// used only when there is no available connection
    fn add_connection(&mut self, db_config: &DbConfig)->Result<(), String>{
        let platform:&str = &db_config.platform;
        match platform{
            "postgres" => {
                    let url = db_config.get_url();
                    let db = Postgres::connect_with_url(&url);
                    match db{
                        Ok(db) => {
                            let platform = Platform::Postgres(db);
                            self.free.push( platform );
                            Ok(())
                        }
                        Err(e) =>{
                            Err(format!("Unable to connect due to {}", e))
                        }
                    }
                },
                "sqlite" => {
                    let url = db_config.get_url();
                    let db = Sqlite::connect_with_url(&url);
                    match db{
                        Ok(db) => {
                            let platform = Platform::Sqlite(db);
                            self.free.push( platform );
                            Ok(())
                        }
                        Err(e) =>{
                            Err(format!("Unable to connect due to {}", e))
                        }
                    }
                },
            _ => panic!("Support for other platform coming..."),
        }
    }
    
    /// exposed api to get connection from a pooled connection
    pub fn get_db_with_url(&mut self, url:&str)->Result<Platform, String>{
        let db_config = DbConfig::from_url(url);
        self.get_db(&db_config)
    }
    
    /// where the pool is checked if there are free connection,
    /// create a new one if nothing is available
    fn get_db(&mut self, db_config:&DbConfig)->Result<Platform, String>{
        let index = self.first_match(db_config);
        match index{
            Some(index) => {
                let platform = self.free.remove(index);
                self.used += 1;
                Ok( platform )
            },
            None => {
                // if no free connection, add a new one then try again
                //println!("no matching connection for {}", db_config.get_url());
                match self.add_connection(db_config){
                    Ok(_) => {
                        self.get_db(db_config)
                    },
                    Err(e) => { 
                        panic!("Unable to get more connections due to :{}",e);
                    }
                }
            }
        }
    }
    /// get first matching database connection from the free pool
    fn first_match(&mut self, db_config:&DbConfig)->Option<usize>{
        let mut index = 0;
        let len = self.total_free_connections();
        for i in 0..len{
            if &self.free[i].as_ref().get_config() == db_config{
                return Some(index);
            }
            index += 1;
        }
        None
    }
    
    /// release the used connection back to the free pool
    pub fn release(&mut self, platform: Platform)->&mut Self{
        //println!("Releasing connection back to the pool");
        self.free.push( platform );
        self.used -= 1;
        self
    }
    pub fn total_connections(&self)->usize{
        self.free.len() + self.used
    }
    
    /// return the number of free available connection in the pool
    pub fn total_free_connections(&self)->usize{
        self.free.len()
    }
    
    /// return the number of free available connection in the pool
    pub fn total_used_connections(&self)->usize{
        self.used
    }
}
