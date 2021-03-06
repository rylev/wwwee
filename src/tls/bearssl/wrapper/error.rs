use super::ffi::*;
use std;

#[derive(Clone, Copy, Debug)]
pub enum Error {
  BadParam = BR_ERR_BAD_PARAM as isize,
  BadState = BR_ERR_BAD_STATE as isize,
  UnsupportedVersion = BR_ERR_UNSUPPORTED_VERSION as isize,
  BadVersion = BR_ERR_BAD_VERSION as isize,
  BadLength = BR_ERR_BAD_LENGTH as isize,
  TooLarge = BR_ERR_TOO_LARGE as isize,
  BadMac = BR_ERR_BAD_MAC as isize,
  NoRandom = BR_ERR_NO_RANDOM as isize,
  UnknownType = BR_ERR_UNKNOWN_TYPE as isize,
  Unexpected = BR_ERR_UNEXPECTED as isize,
  BadCcs = BR_ERR_BAD_CCS as isize,
  BadAlert = BR_ERR_BAD_ALERT as isize,
  BadHandshake = BR_ERR_BAD_HANDSHAKE as isize,
  OversizedId = BR_ERR_OVERSIZED_ID as isize,
  BadCipherSuite = BR_ERR_BAD_CIPHER_SUITE as isize,
  BadCompression = BR_ERR_BAD_COMPRESSION as isize,
  BadFraglen = BR_ERR_BAD_FRAGLEN as isize,
  BadSecreneg = BR_ERR_BAD_SECRENEG as isize,
  ExtraExtension = BR_ERR_EXTRA_EXTENSION as isize,
  BadSni = BR_ERR_BAD_SNI as isize,
  BadHelloDone = BR_ERR_BAD_HELLO_DONE as isize,
  LimitExceeded = BR_ERR_LIMIT_EXCEEDED as isize,
  BadFinished = BR_ERR_BAD_FINISHED as isize,
  ResumeMismatch = BR_ERR_RESUME_MISMATCH as isize,
  InvalidAlgorithm = BR_ERR_INVALID_ALGORITHM as isize,
  BadSignature = BR_ERR_BAD_SIGNATURE as isize,
  WrongKeyUsage = BR_ERR_WRONG_KEY_USAGE as isize,
  NoClientAuth = BR_ERR_NO_CLIENT_AUTH as isize,
  Io = BR_ERR_IO as isize,
  RecvFatalAlert = BR_ERR_RECV_FATAL_ALERT as isize,
  SendFatalAlert = BR_ERR_SEND_FATAL_ALERT as isize,
}

impl Error {
  pub fn as_io_error(self, msg: &'static str) -> std::io::Error {
    println!("as_io_error from {:?}", self);
    std::io::Error::new(std::io::ErrorKind::Other, msg)
  }
}
