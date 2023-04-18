mod controller;
mod server;

fn main() -> std::io::Result<()> {
    server::server(("192.168.1.218", 9000))
}
