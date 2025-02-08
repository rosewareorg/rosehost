use crate::file::File;
use ekero::prelude::*;

pub struct Html {
    pub data: File,
}

impl Resource for Html {
    fn add_headers(&self, source: Response) -> Response {
        source.header("Content-Type", "text/html")
    }

    fn write_to_stream(&self, stream: &mut std::net::TcpStream) -> std::io::Result<()> {
        self.data.write_to_stream(stream)
    }
}

impl Html {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: File { data },
        }
    }
}
