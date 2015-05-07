use std::io;
use std::collections::HashMap;

use net::Url;

// TODO: Types RequestBytes, ResponseBytes ?

pub struct HttpReplayer {
    context: &'static str,
    recordings: HashMap<(Url, Vec<u8>), Vec<u8>>
}

impl HttpReplayer {
    pub fn new(context: &'static str) -> HttpReplayer {
        HttpReplayer { context: context, recordings: HashMap::new() }
    }

    pub fn record_response(&mut self, url: Url, request: Vec<u8>, response: Vec<u8>) {
        self.recordings.insert((url, request), response);
    }

    pub fn replay_response(&mut self, url: Url, request: Vec<u8>) -> Option<&Vec<u8>> {
        self.recordings.get(&(url, request))
    }
}

impl Drop for HttpReplayer {
    fn drop(&mut self) {
        // println!("I'm dying! {:?}", self.recordings);
        drop(&mut self.recordings);
    }
}
