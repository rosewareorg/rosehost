use ekero::prelude::*;
use std::io::Write;

pub struct File {
    pub data: Vec<u8>,
}

impl Resource for File {
    #[inline]
    fn add_headers(&self, source: Response) -> Response {
        source
    }

    fn write_to_stream(&self, stream: &mut std::net::TcpStream) -> std::io::Result<()> {
        let _size = stream.write(&self.data)?;
        Ok(())
    }
}
