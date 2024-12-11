/**
 * Database metadata information extracted from the configuration collection.
 * Details look at comment above [key].
 **/
use super::Config;

#[derive(Debug)]
pub struct Metadata {
    pub credential: MetadataDetail,
    pub host: MetadataDetail,
    pub db_name: String,
}

#[derive(Debug)]
pub enum MetadataDetail {
    Credential {
        username: String,
        password: String,
    },
    Host {
        name: String,
        port: Option<u16>,
    },
}

impl Metadata {

    /**
     * Available config keys look at [key]
     **/
    pub fn from_config(config: &Config) -> Metadata {
        let credential = MetadataDetail::credential(config);
        let host = MetadataDetail::host(config);
        let db_name = config.name();

        Metadata { credential, host, db_name }
    }

}

impl MetadataDetail {

    pub fn credential(config: &Config) -> Self {
        let username = config.username();
        let password = config.password();

        Self::Credential { username, password }
    }

    pub fn host(config: &Config) -> Self {
        let name = config.host();
        let port = config.port();

        Self::Host { name, port }
    }

}

/**
 * MongoDB access credential: [username] and [password].
 * Where [username] = "database.credential.usr"
 *       [password] = "database.credential.pwd"
 *
 * MongoDB host address: [name] and [port].
 * Where [name] = "database.host.name"
 *       [port] = "database.host.port"
 *
 * MongoDB database name: [db_name].
 * Where [db_name] = "database.db.name"
 **/
mod key {

    pub fn username() -> Vec<String> {
        vec!["database".to_string(), "credential".to_string(), "usr".to_string()]
    }

    pub fn password() -> Vec<String> {
        vec!["database".to_string(), "credential".to_string(), "pwd".to_string()]
    }

    pub fn host() -> Vec<String> {
        vec!["database".to_string(), "host".to_string(), "name".to_string()]
    }

    pub fn port() -> Vec<String> {
        vec!["database".to_string(), "host".to_string(), "port".to_string()]
    }

    pub fn db_name() -> Vec<String> {
        vec!["database".to_string(), "db".to_string(), "name".to_string()]
    }

}

trait MetadataConfig {
    fn username(&self) -> String;
    fn password(&self) -> String;
    fn host(&self) -> String;
    fn port(&self) -> Option<u16>;
    fn name(&self) -> String;
}

impl MetadataConfig for Config {

    fn username(&self) -> String {
        self[key::username()].clone()
    }

    fn password(&self) -> String {
        self[key::password()].clone()
    }

    fn host(&self) -> String {
        self[key::host()].clone()
    }

    fn port(&self) -> Option<u16> {
        self.get(key::port())
            .cloned()
            .map(|port| {
                match port.parse::<u16>() {
                    Ok(port) => port,
                    Err(_) => panic!(r#"Panic: Failed to parse port value "{}"."#, port),
                }
            })
    }

    fn name(&self) -> String {
        self[key::db_name()].clone()
    }

}