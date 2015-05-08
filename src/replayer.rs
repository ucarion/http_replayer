use std::collections::HashMap;
use std::io;
use std::fs::{self, File, PathExt};
use std::path::{Path, PathBuf};

use net::Url;

// TODO: Types RequestBytes, ResponseBytes ?

pub struct HttpReplayer {
    context: String,
    stream_type: StreamType,
    recordings: HashMap<String, Vec<u8>>
}

#[derive(Debug, Eq, PartialEq)]
enum StreamType {
    Record,
    Replay
}

impl HttpReplayer {
    pub fn new(context: &str) -> HttpReplayer {
        if HttpReplayer::serialization_path_exists(context) {
            HttpReplayer {
                context: context.to_string(),
                stream_type: StreamType::Replay,
                recordings: HashMap::new()
            }
        } else {
            HttpReplayer {
                context: context.to_string(),
                stream_type: StreamType::Record,
                recordings: HashMap::new()
            }
        }
    }

    pub fn load_stream(&mut self, url: Url, request: Vec<u8>) -> Option<&Vec<u8>> {
        None
    }

    fn record_response(&mut self, url: Url, request: Vec<u8>, response: Vec<u8>) {
        let request = HttpReplayer::encode_request(&url, &request);
        self.recordings.insert(request, response);
    }

    fn replay_response(&mut self, url: Url, request: Vec<u8>) -> Option<&Vec<u8>> {
        let request = HttpReplayer::encode_request(&url, &request);
        self.recordings.get(&request)
    }

    fn encode_request(url: &Url, request: &Vec<u8>) -> String {
        format!("{}:{:?}", url, request)
    }

    fn serialization_path_exists(context: &str) -> bool {
        fs::metadata(HttpReplayer::serialization_path_for(context)).is_ok()
    }

    fn serialization_path_for(context: &str) -> PathBuf {
        let suffix = format!("{}.json", context);
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
    let actual = HttpReplayer::serialization_path_for("foobar");
    let expected = "./fixtures/http_replayer/foobar.json";

    assert_eq!(expected, actual.to_str().unwrap());
}

#[test]
fn test_initialize() {
    let replayer = HttpReplayer::new("does-not-exist");
    assert_eq!(StreamType::Record, replayer.stream_type);

    fs::create_dir_all("./fixtures/http_replayer").unwrap();
    let mut f = File::create("./fixtures/http_replayer/does-exist.json").unwrap();
    // f.write_all("{}")

    let replayer = HttpReplayer::new("does-exist");
    assert_eq!(StreamType::Replay, replayer.stream_type);
}

#[test]
fn test_encode_request() {
    let url = Url { host: "example.com".to_string(), port: 80, scheme: "http".to_string() };
    let data = vec![1, 2, 3];

    assert_eq!("http://example.com:80:[1, 2, 3]", HttpReplayer::encode_request(&url, &data));
}
