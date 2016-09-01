use url::{Url, Host};



#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct DbConfig {
    /// postgres, sqlite, mysql
    /// some fields are optional since sqlite is not applicable for those
    pub platform: String,
    pub username: Option<String>,
    pub password: Option<String>,
    /// localhost
    pub host: Option<String>,
    /// 5432
    pub port: Option<u16>,
    pub database: String,
    pub ssl: bool,
}

impl DbConfig {
    /// TODO: get rid of the hacky way parsing database url
    /// https://github.com/servo/rust-url/issues/40
    pub fn from_url(url: &str) -> Option<Self> {
        let parsed = Url::parse(url);
        match parsed {
            Ok(parsed) => {
                Some(DbConfig {
                    platform: parsed.scheme().to_owned(),
                    username: {
                        let username = parsed.username();
                        if username.is_empty() {
                            None
                        } else {
                            Some(username.to_owned())
                        }
                    },
                    password: parsed.password().map(|s| s.to_owned()),
                    host: {
                        let host_str = parsed.host_str();
                        match host_str {
                            Some(ref host_str) => {
                                if host_str.is_empty() {
                                    None
                                } else {
                                    Some(host_str.to_string())
                                }
                            }
                            None => None,
                        }
                    },
                    port: parsed.port(),
                    database: parsed.path().to_string().trim_left_matches("/").to_owned(),
                    ssl: false,
                })
            }
            Err(e) => None,
        }
    }

    pub fn get_url(&self) -> String {
        let mut url = String::new();
        url.push_str(&self.platform.to_owned());
        url.push_str("://");

        if let Some(ref username) = self.username {
            url.push_str(username);
        }

        if let Some(ref password) = self.password {
            url.push_str(":");
            url.push_str(password);
        }

        if let Some(ref host) = self.host {
            url.push_str("@");
            url.push_str(&host);
        }

        if let Some(ref port) = self.port {
            url.push_str(":");
            url.push_str(&format!("{}", port));
        }

        url.push_str("/");
        url.push_str(&self.database);
        url
    }
}

#[test]
fn test_config_url() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let config = DbConfig {
        platform: "postgres".to_owned(),
        username: Some("postgres".to_owned()),
        password: Some("p0stgr3s".to_owned()),
        host: Some("localhost".to_owned()),
        port: None,
        ssl: false,
        database: "bazaar_v8".to_owned(),
    };

    assert_eq!(config.get_url(), url.to_owned());
}

#[test]
fn test_config_from_url() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let config = DbConfig::from_url(url).unwrap();
    assert_eq!(config.get_url(), url.to_owned());
}


#[test]
fn test_config_url_with_port() {
    let url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";
    let config = DbConfig {
        platform: "postgres".to_owned(),
        username: Some("postgres".to_owned()),
        password: Some("p0stgr3s".to_owned()),
        host: Some("localhost".to_owned()),
        port: Some(5432),
        database: "bazaar_v8".to_owned(),
        ssl: false,
    };

    assert_eq!(config.get_url(), url.to_owned());
}

#[test]
fn test_config_sqlite_url_with_port() {
    let url = "sqlite:///bazaar_v8.db";
    let parsed_config = DbConfig::from_url(url).unwrap();
    let expected_config = DbConfig {
        platform: "sqlite".to_owned(),
        username: None,
        password: None,
        host: None,
        port: None,
        database: "bazaar_v8.db".to_owned(),
        ssl: false,
    };
    println!("{:?}", parsed_config);
    assert_eq!(parsed_config, expected_config);
}


#[test]
fn test_config_sqlite_url_with_path() {
    let url = "sqlite:///home/some/path/file.db";
    let parsed_config = DbConfig::from_url(url).unwrap();
    let expected_config = DbConfig {
        platform: "sqlite".to_owned(),
        username: None,
        password: None,
        host: None,
        port: None,
        database: "home/some/path/file.db".to_owned(),
        ssl: false,
    };
    println!("{:?}", parsed_config);
    assert_eq!(parsed_config, expected_config);
}


#[test]
fn sqlite_in_memory() {
    let url = "sqlite:///:memory:";
    let parsed_config = DbConfig::from_url(url).unwrap();
    let expected_config = DbConfig {
        platform: "sqlite".to_owned(),
        username: None,
        password: None,
        host: None,
        port: None,
        database: ":memory:".to_owned(),
        ssl: false,
    };
    println!("{:?}", parsed_config);
    assert_eq!(parsed_config, expected_config);
}
