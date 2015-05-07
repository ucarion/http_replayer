use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

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

    fn serialization_path(&self) -> PathBuf {
        let suffix = format!("{}.json", self.context);
        Path::new(".").join("fixtures").join("http_replayer").join(suffix)
    }
}

impl Drop for HttpReplayer {
    fn drop(&mut self) {
        drop(&mut self.recordings);
    }
}


#[test]
fn test_serialization_path() {
    let replayer = HttpReplayer::new("foobar");
    let expected = "./fixtures/http_replayer/foobar.json";

    assert_eq!(expected, replayer.serialization_path().to_str().unwrap());
}
