use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write, Cursor};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use hyper::net::{NetworkStream, NetworkConnector};

#[derive(Clone, Debug)]
pub struct MockStream {
    read: Option<Cursor<Vec<u8>>>,
    write: Vec<u8>,

    context: &'static str,
    host: String,
    scheme: String,
    port: u16
}

impl MockStream {
    fn new(host: String, port: u16, scheme: String, context: &'static str) -> MockStream {
        MockStream {
            read: None,
            write: vec![],
            host: host,
            scheme: scheme,
            port: port,
            context: context
        }
    }

    fn recorded_response(&self) -> Option<Cursor<Vec<u8>>> {
        None
    }

    fn find_response<'a>(&self, responses: &'a [RecordedResponse]) -> Option<&'a RecordedResponse> {
        responses.iter().find(|&response| { response.url == self.get_url() })
    }

    fn get_url(&self) -> String {
        format!("{}://{}:{}", self.scheme, self.host, self.port)
    }

    fn record_response(&self) {
    }

    fn recorded_response_path(&self) -> PathBuf {
        Path::new(".")
            .join("fixtures")
            .join("net_replayer")
            .join(format!("{}{}", self.context, ".json"))
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(ref mut cursor) = self.read {
            cursor.read(buf)
        } else if let Some(cursor) = self.recorded_response() {
            self.read = Some(cursor);
            self.read(buf)
        } else {
            self.record_response();
            self.read(buf)
        }
    }
}

impl Write for MockStream {
    fn write(&mut self, msg: &[u8]) -> io::Result<usize> {
        Write::write(&mut self.write, msg)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl NetworkStream for MockStream {
    fn peer_addr(&mut self) -> io::Result<SocketAddr> {
        Ok("127.0.0.1:1337".parse().unwrap())
    }
}

struct MockRedirectPolicy {
    context: &'static str
}

impl MockRedirectPolicy {
    fn new(context: &'static str) -> MockRedirectPolicy {
        MockRedirectPolicy { context: context }
    }
}

pub struct RecordedResponse {
    url: String,
    sent: Vec<u8>,
    received: Vec<u8>
}

impl NetworkConnector for MockRedirectPolicy {
    type Stream = MockStream;

    fn connect(&mut self, host: &str, port: u16, scheme: &str) -> io::Result<MockStream> {
        Ok(MockStream::new(host.to_string(), port, scheme.to_string(), self.context))
    }
}

#[test]
fn test_mock_stream_path() {
    let stream = MockStream::new(
        "127.0.0.1".to_string(), 80, "http".to_string(), "my_test");

    assert_eq!("./fixtures/net_replayer/my_test.json",
               stream.recorded_response_path().to_str().unwrap());
}
