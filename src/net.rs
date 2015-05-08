use std::io::{self, Write};
use std::fmt;

use hyper::net::{HttpConnector, NetworkConnector};
use hyper::client::Response;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Url {
    pub host: String,
    pub port: u16,
    pub scheme: String,
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}://{}:{}", self.scheme, self.host, self.port)
    }
}

pub fn fetch_http(url: &Url, data: &[u8]) -> io::Result<Vec<u8>> {
    let mut connector = HttpConnector(None);
    let mut stream = try!(connector.connect(&url.host, url.port, &url.scheme));

    try!(stream.write(data));

    let mut buf = Vec::new();
    let mut res = Response::new(Box::new(stream)).unwrap();
    try!(write!(&mut buf, "{} {}\r\n", res.version, res.status));
    try!(write!(&mut buf, "{}\r\n", res.headers));
    try!(io::copy(&mut res, &mut buf));

    Ok(buf)
}

#[test]
fn test_display() {
    let url = Url { host: "example.com".to_string(), port: 80, scheme: "http".to_string() };
    assert_eq!("http://example.com:80", url.to_string());
}

#[test]
fn test_fetch_http() {
    // let msg = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";
    // let res = fetch_http("example.com", 80, "http", msg).unwrap();

    // println!("{:?}", res);

    // TODO: Start a hyper server and test against local server.
}
