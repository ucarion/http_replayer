use std::io::{self, Read, Write, Cursor};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use hyper::net::{NetworkStream, NetworkConnector};

use net::{self, Url};

struct MockConnector {
    replayer: ResponseReplayer,
}

impl NetworkConnector for MockConnector {
    type Stream = MockStream;

    fn connect(&mut self, host: &str, port: u16, scheme: &str) -> io::Result<MockStream> {
        let replayer = ResponseReplayer { context: "hello" };
        let arc = Arc::new(Mutex::new(replayer));
        let stream_type = StreamType::Record;

        Ok(MockStream {
            url: Url { host: host.to_string(), port: port, scheme: scheme.to_string() },
            stream_type: stream_type,
            replayer: arc,

            read: None,
            write: vec![]
        })
    }
}

struct ResponseReplayer {
    context: &'static str
}

impl ResponseReplayer {
    fn record_response(&mut self, url: &Url, data: &[u8]) {

    }

    fn replay_response(&mut self, url: &Url, data: &[u8]) -> io::Result<Vec<u8>> {
        Ok(vec![])
    }
}

#[derive(Clone)]
struct MockStream {
    url: Url,
    stream_type: StreamType,
    replayer: Arc<Mutex<ResponseReplayer>>,

    read: Option<Cursor<Vec<u8>>>,
    write: Vec<u8>
}

impl MockStream {
    fn load_stream(&mut self) {
        let mut replayer = self.replayer.lock().unwrap();

        match self.stream_type {
            StreamType::Record => {
                let actual_res = net::fetch_http(&self.url, &self.write).ok()
                    .expect("Failed to record actual HTTP");
                replayer.record_response(&self.url, &actual_res);
                self.read = Some(Cursor::new(actual_res));
            }

            StreamType::Replay => {
                let res = replayer.replay_response(&self.url, &self.write).ok()
                    .expect("Failed to replay HTTP");
                self.read = Some(Cursor::new(res));
            }
        }
    }
}

#[derive(Clone)]
enum StreamType {
    Record,
    Replay
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(ref mut read) = self.read {
            read.read(buf)
        } else {
            self.load_stream();
            self.read(buf)
        }
    }
}

impl Write for MockStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.write.flush()
    }
}

impl NetworkStream for MockStream {
    fn peer_addr(&mut self) -> io::Result<SocketAddr> {
        Ok("127.0.0.1:1337".parse().unwrap())
    }
}

#[test]
fn test_normal_usage() {
    // TODO: This works, but it should be testing against a local server instead
    // of example.com.

    // use hyper::Client;

    // let replayer = ResponseReplayer { context: "test" };
    // let connector = MockConnector { replayer: replayer };

    // // Create a client.
    // let mut client = Client::with_connector(connector);

    // // Creating an outgoing request.
    // let mut res = client.get("http://www.example.com/")
    //     // let 'er go!
    //     .send().unwrap();

    // // Read the Response.
    // let mut body = String::new();
    // res.read_to_string(&mut body).unwrap();

    // println!("Response: {}", body);
    // panic!();
}
