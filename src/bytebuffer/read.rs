pub struct ReadByteBuffer {
    buf: Vec<u8>,
    index: usize,
}

impl ReadByteBuffer {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            buf: bytes.to_vec(),
            index: 0,
        }
    }

    pub fn new() -> Self {
        Self {
            buf: vec![],
            index: 0,
        }
    }
    pub fn write_u8(&mut self, n: u8) {
        self.buf.push(n);
    }

    pub fn write_u16(&mut self, n: u16) {
        self.buf
            .extend_from_slice(&[(n >> 8) as u8, (n & 0xFF) as u8])
    }

    pub fn write_u32(&mut self, n: u32) {
        self.buf.extend_from_slice(&[
            ((n >> 24) & 0xFF) as u8,
            ((n >> 16) & 0xFF) as u8,
            ((n >> 8) & 0xFF) as u8,
            (n & 0xFF) as u8,
        ])
    }

    pub(crate) fn into_vec(self) -> Vec<u8> {
        self.buf
    }
}
