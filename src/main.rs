use serbder::*;

fn main() {
    let server = Server::new("localhost:6666");
    server.run(|| {})
}
