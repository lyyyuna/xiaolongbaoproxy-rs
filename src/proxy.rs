pub struct Proxy<'a> {
    host: &'a str,
    port: i32
}

impl<'a> Proxy<'a> {
    pub fn new(host: &'a str, port: i32) -> Self {
        Proxy {
            host: host,
            port: port
        }
    }

    pub fn serve(&self) {
        println!("serving on: {}:{}", self.host, self.port);
    }
}