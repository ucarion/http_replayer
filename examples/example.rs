extern crate hyper;
extern crate http_replayer;

use std::io::Read;

use hyper::Client;
use http_replayer::mock::MockConnector;

fn main() {
    let mut client = Client::with_connector(MockConnector::new("testing"));

    let mut res = client.get("http://www.example.com/").send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}
