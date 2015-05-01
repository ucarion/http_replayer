use std::io::{self, Read, Write, Cursor};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use hyper::net::{NetworkStream, NetworkConnector};

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

#[derive(Clone)]
struct MockStream {
    stream_type: StreamType,
    replayer: Arc<Mutex<ResponseReplayer>>,

    read: Option<Cursor<Vec<u8>>>,
    write: Vec<u8>
}

impl MockStream {
    fn load_stream(&mut self) {
        let replayer = self.replayer.lock().unwrap();

        match self.stream_type {
            StreamType::Record => {
                // TODO: Record a response, let the replayer know, and save it
                // as the current read.
            }

            StreamType::Replay => {
                // TODO: Ask the replayer for a response, and save it as the
                // current read.
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
fn it_works() {
    assert!(false);
}
