use std::io;

pub struct Buffer {
  vec: Vec<u8>
}

impl Buffer {

  pub fn new() -> Buffer {
    Buffer {vec: Vec::with_capacity(4096)}
  }

  pub fn clear(&mut self) {
    self.vec.clear()
  }

  pub fn len(&self) -> usize {
    self.vec.len()
  }

  pub fn capacity(&self) -> usize {
    self.vec.capacity()
  }

  pub unsafe fn set_len(&mut self, len: usize) {
    if len <= self.capacity() {
      self.vec.set_len(len);
    }
  }

  pub fn as_slice<'a>(&'a self) -> &'a [u8] {
    self.vec.as_slice()
  }

  pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [u8] {
    self.vec.as_mut_slice()
  }

  pub fn read_available<R: io::Read>(&mut self, reader: &mut R) -> io::Result<usize> {
    let len_before = self.vec.len();
    match reader.read_to_end(&mut self.vec) {
      Ok(bytes_read) => Ok(bytes_read),
      Err(err) => {
        match err.kind() {
          io::ErrorKind::WouldBlock => Ok(self.vec.len() - len_before),
          _ => Err(err)
        }
      }
    }
  }
}

impl io::Write for Buffer {
  fn write(&mut self, src: &[u8]) -> io::Result<usize> {
    self.vec.write(src)
  }

  fn flush(&mut self) -> io::Result<()> {
    self.vec.flush()
  }
}

#[cfg(test)]
mod tests {
  use super::Buffer;
  use std::io::Write;

  #[test]
  fn test_write() {
    let mut buffer = Buffer::new();
    assert_eq!(buffer.as_slice(), b"");
    write!(buffer, "hello {}", 1).unwrap();
    assert_eq!(buffer.as_slice(), b"hello 1");
  }

  #[test]
  fn test_clear() {
    let mut buffer = Buffer::new();
    write!(buffer, "hello").unwrap();
    buffer.clear();
    assert_eq!(buffer.as_slice(), b"");
  }

  #[test]
  fn test_read_available() {
    let data = b"hello";
    let mut buffer = Buffer::new();
    let res = buffer.read_available(&mut data.as_ref());
    assert_eq!(res.ok(), Some(5));
    assert_eq!(buffer.len(), 5);
    assert_eq!(buffer.as_slice(), b"hello");
  }
}
