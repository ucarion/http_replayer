use std::io;

use net::Url;

pub struct HttpReplayer {
    context: &'static str
}

impl HttpReplayer {
    pub fn new(context: &'static str) -> HttpReplayer {
        HttpReplayer { context: context }
    }

    pub fn record_response(&mut self, url: &Url, response: &[u8]) {

    }

    pub fn replay_response(&mut self, url: &Url, request: &[u8]) -> io::Result<Vec<u8>> {
        Ok(vec![])
    }
}
