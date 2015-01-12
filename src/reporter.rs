/// I am sure we should be able to do this more efficiently
/// i.e. without building url string each time

use hyper;

pub struct Reporter {
    client: influx::Client,
    protocol: String, // http(s)
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String
}

impl Reporter {
    pub fn new(protocol: String, host: String, port: u16) -> Reporter {
        Reporter {
            client: hyper::Client::new(),
            protocol: protocol,
            host: host,
            port: port,
            username: username,
            password: password
        }
    }

    fn url
}
