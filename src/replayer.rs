use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::fs::{self, File, PathExt, OpenOptions};
use std::path::{Path, PathBuf};

use rustc_serialize::json;

use net::Url;

// TODO: Types RequestBytes, ResponseBytes ?

#[derive(Debug)]
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
        let (stream_type, recordings) = if HttpReplayer::serialization_path_exists(context) {
            (StreamType::Replay, HttpReplayer::load_recordings(context))
        } else {
            (StreamType::Record, HashMap::new())
        };

        HttpReplayer {
            context: context.to_string(),
            stream_type: stream_type,
            recordings: recordings
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

    fn dump_recordings(&self) {
        let recordings = json::encode(&self.recordings).ok()
            .expect("Could not encode recordings to JSON");
        let path = HttpReplayer::serialization_path_for(&self.context);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path).ok()
            .expect("Could not open serialization file");

        file.write_all(recordings.as_bytes()).unwrap();
    }

    fn load_recordings(context: &str) -> HashMap<String, Vec<u8>> {
        let path = HttpReplayer::serialization_path_for(context);
        let mut file = File::open(path).ok()
            .expect("Could not open serialization file");

        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        json::decode(&s).unwrap()
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
        self.dump_recordings();

        drop(&mut self.recordings);
        drop(&mut self.context);
        drop(&mut self.stream_type);
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

// TODO: Clean up this test in such a way that it can really prove the replayer's Drop is properly
// called and whatnot, and do so without causing side effects.
//
// #[test]
// fn it_works() {
//     {
//         let mut replayer = HttpReplayer::new("ulysse");
//         replayer.record_response(Url {
//             host: "example.com".to_string(),
//             port: 80,
//             scheme: "http".to_string()
//         }, vec![1, 2, 3], vec![4, 5, 6]);
//     } // replayer dies here
//
//     let replayer = HttpReplayer::new("ulysse");
//     println!("{:?}", replayer);
//
//     panic!();
// }
