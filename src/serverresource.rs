use ekero::prelude::*;

pub struct ServerResource<T: Resource> {
    resource: T,
}

impl<T: Resource> Resource for ServerResource<T> {
    fn add_headers(&self, source: Response) -> Response {
        self.resource
            .add_headers(source)
            .header("Server", "Rosehost/0.1.4")
    }

    fn write_to_stream(&self, stream: &mut std::net::TcpStream) -> std::io::Result<()> {
        self.resource.write_to_stream(stream)
    }
}

impl<T: Resource> ServerResource<T> {
    pub fn new(t: T) -> Self {
        Self { resource: t }
    }
}
