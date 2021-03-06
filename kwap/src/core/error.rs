use kwap_msg::to_bytes::MessageToBytesError;
use kwap_msg::MessageParseError;
use no_std_net::SocketAddr;

use crate::net::Socket;
use crate::platform::Platform;

/// The context that an error occurred in
#[derive(Debug, Clone, Copy)]
pub enum When {
  /// We were polling for a message when the error occurred
  Polling,
  /// We were sending a message
  SendingMessage(Option<SocketAddr>, kwap_msg::Id, kwap_msg::Token),
  /// Not sure that `When` is valuable anymore
  None,
}

impl When {
  /// Construct a specific error from the context the error occurred in
  pub fn what<P: Platform>(self, what: What<P>) -> Error<P> {
    Error { when: self, what }
  }
}

/// An error encounterable from within Core
#[derive(Debug)]
pub struct Error<P: Platform> {
  /// What happened?
  pub what: What<P>,
  /// What were we doing when it happened?
  pub when: When,
}

impl<P: Platform> Error<P> {
  /// Is this error `FromBytes`?
  pub fn message_parse_error(&self) -> Option<&MessageParseError> {
    match self.what {
      | What::FromBytes(ref e) => Some(e),
      | _ => None,
    }
  }
}

/// A contextless error with some additional debug data attached.
#[derive(Debug)]
pub enum What<P: Platform> {
  /// Some socket operation (e.g. connecting to host) failed
  SockError(<<P as Platform>::Socket as Socket>::Error),
  /// Serializing a message from bytes failed
  FromBytes(MessageParseError),
  /// Serializing a message to bytes failed
  ToBytes(MessageToBytesError),
  /// Uri-Host in request was not a utf8 string
  HostInvalidUtf8(core::str::Utf8Error),
  /// Uri-Host in request was not a valid IPv4 address (TODO)
  HostInvalidIpAddress,
  /// A CONfirmable message was sent many times without an ACKnowledgement.
  MessageNeverAcked,
  /// The clock failed to provide timing.
  ///
  /// See [`embedded_time::clock::Error`]
  ClockError,
  /// Something timed out
  Timeout,
}
