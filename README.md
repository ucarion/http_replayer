# http_replayer

`http_replayer` is a level of middleware for [hyper][1] that replays HTTP server responses. This lets your tests be 
deterministic, and not have to rely on (or wait for) the network to run your tests. This project is similar to the Ruby
project [vcr][2], if you've used something like that before.

You could probably have written this library yourself. It simply maintains a `HashMap` from (URL, Request) pairs to server
responses. If sending the same request to the same server multiple times always produces the same response from the server,
then you can use `http_replayer`.

## Usage

Here's an example in code:

```rust
extern crate hyper;
extern crate http_replayer;

use std::io::Read;

use hyper::Client;
use http_replayer::mock::MockConnector;

fn main() {
    // Create a client.
    //
    // Normally (and in production), you would do this:
    //
    // let mut client = Client::new();
    //
    // But with http_replayer (in your tests), you write:
    let mut client = Client::with_connector(MockConnector::new("testing")); // the "testing" is explained below

    // Creating an outgoing request.
    //
    // The first time you run this, Hyper will actually send out a request to the internet, but every time
    // thereafter a locally saved response will be returned instead.
    let mut res = client.get("http://www.example.com/")
        // let 'er go!
        .send().unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}
```

A `MockConnector` plugs into hyper's [`NetworkConnector`][3] interface to do its' magic. `MockConnector#new` accepts a
`context: &str` as argument -- using unique contexts allows you to use multiple `http_replayer` sessions concurrently.

## Warnings

`http_replayer` is very new right now. You should not be using it in production.

[1]: https://github.com/hyperium/hyper
[2]: https://github.com/vcr/vcr
[3]: http://hyperium.github.io/hyper/hyper/net/trait.NetworkConnector.html
