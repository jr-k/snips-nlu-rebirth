use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub global: ConfigGlobal,
    pub mqtt: ConfigMqtt
}

#[derive(Deserialize)]
pub struct ConfigGlobal {
    pub engine_dir: String
}


#[derive(Deserialize)]
pub struct ConfigMqtt {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub tls: bool,
    pub ssl_cert: String,
    pub ssl_cert_chain: String,
    pub ssl_key: String
}
