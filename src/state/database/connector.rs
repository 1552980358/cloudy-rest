use mongodb::{
    error::ErrorKind,
    options::{ClientOptions, Credential, ServerAddress, ServerApi, ServerApiVersion},
    Client,
};

use super::metadata::{Metadata, MetadataDetail};

pub fn with_metadata(metadata: &Metadata) -> Client {
    let client_options = client_option(metadata);

    match Client::with_options(client_options) {
        Ok(client) => client,
        Err(err) => {
            let msg = match *err.kind {
                ErrorKind::Authentication { message, .. } => {
                    format!(r#"Authentication error "{}""#, message)
                }
                ErrorKind::Internal { message, .. } => format!(r#"Internal error "{}""#, message),
                ErrorKind::Io(_) => "I/O error".to_string(),
                ErrorKind::Shutdown => "Connection shutdown".to_string(),
                _ => "Unknown error".to_string(),
            };
            panic!("Panic: Failed to connect to the database ({}).", msg);
        }
    }
}

fn client_option(metadata: &Metadata) -> ClientOptions {
    ClientOptions::builder()
        .server_api(server_api())
        .hosts(vec![server_address(&metadata.host)])
        .credential(credential(&metadata.credential))
        .build()
}

fn credential(credential: &MetadataDetail) -> Credential {
    let MetadataDetail::Credential { username, password } = credential else {
        panic!(
            r#"Panic: MetadataDetail::Credential expected, but found another variant {:?}."#,
            credential
        );
    };

    Credential::builder()
        .username(username.clone())
        .password(password.clone())
        .build()
}

fn server_api() -> ServerApi {
    ServerApi::builder()
        .version(ServerApiVersion::V1)
        .strict(true)
        .build()
}

fn server_address(host: &MetadataDetail) -> ServerAddress {
    let MetadataDetail::Host { name, port } = host else {
        panic!(
            r#"Panic: MetadataDetail::Host expected, but found another variant {:?}."#,
            host
        );
    };

    ServerAddress::Tcp {
        host: name.clone(),
        port: port.clone(),
    }
}
