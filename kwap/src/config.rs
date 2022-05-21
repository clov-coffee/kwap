use core::ops::Range;

use embedded_time::duration::Milliseconds;
use kwap_macros::rfc_7252_doc;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ConfigData {
  pub(crate) token_seed: u16,
  pub(crate) ack_timeout_max_millis: u32,
  pub(crate) ack_timeout_min_millis: u32,
  pub(crate) default_leisure_millis: u32,
  pub(crate) max_retransmit_attempts: u16,
  pub(crate) nstart: u8,
  pub(crate) probing_rate_bytes_per_sec: u16,
}

/// CoAP runtime config
///
/// Allows you to configure things like
/// "how many concurrent requests are we allowed
/// to send?" and "how long should we wait to resend
/// unacknowledged confirmable requests?"
///
/// For an example see [`Config::new`].
#[derive(Debug, Default, Clone, Copy)]
pub struct Config {
  token_seed: Option<u16>,
  ack_timeout_max_millis: Option<u32>,
  ack_timeout_min_millis: Option<u32>,
  default_leisure_millis: Option<u32>,
  max_retransmit_attempts: Option<u16>,
  nstart: Option<u8>,
  probing_rate_bytes_per_sec: Option<u16>,
}

/// Bytes / Second
#[derive(Debug, Clone, Copy)]
pub struct BytesPerSecond(pub u16);

impl Config {
  /// Creates a new (empty) runtime config
  ///
  /// ```
  /// use embedded_time::Milliseconds;
  /// use kwap::config::{BytesPerSecond, Config};
  /// use kwap::retry::Attempts;
  ///
  /// let config = Config::new().token_seed(35718)
  ///                           .max_concurrent_requests(35718)
  ///                           .probing_rate(BytesPerSecond(10_000))
  ///                           .max_con_request_retries(Attempts(10))
  ///                           .ack_timeout(Milliseconds::<u64>(10_000))
  ///                           .ack_random_factor(1.5);
  /// ```
  pub fn new() -> Self {
    Default::default()
  }

  /// Set the seed used to generate message [`Token`](kwap_msg::Token)s.
  ///
  /// The default value is 0, although it is
  /// best practice to set this to something else.
  /// This could be a random integer, or a machine identifier.
  ///
  /// _e.g. if you're developing a swarm of
  /// smart CoAP-enabled thermostats, each one would ideally
  /// have a distinct token_seed._
  ///
  /// The purpose of the seed is to make it more
  /// difficult for an observer of unencrypted
  /// CoAP traffic to guess what the next token will be.
  ///
  /// Tokens are generated by smooshing together
  /// the 2-byte seed with an 8-byte timestamp from
  /// the system clock.
  ///
  /// ```text
  /// Core.token_seed
  /// ||
  /// xx xxxxxxxx
  ///    |      |
  ///    timestamp
  /// ```
  ///
  /// Then a hashing algorithm is used to make it opaque and
  /// reduce the size to 8 bytes.
  pub fn token_seed(mut self, token_seed: u16) -> Self {
    self.token_seed = Some(token_seed);
    self
  }

  /// Set the transmission rate that we should do our best
  /// not to exceed when waiting for:
  /// - responses to our NON requests
  /// - responses to our acked CON requests
  ///
  /// The default value is 1,000 (1KB)
  pub fn probing_rate(mut self, probing_rate: BytesPerSecond) -> Self {
    self.probing_rate_bytes_per_sec = Some(probing_rate.0);
    self
  }

  /// Set the number of concurrent requests we are allowed
  /// to have in-flight for each server.
  ///
  /// The default value is 1 (no concurrency)
  pub fn max_concurrent_requests(mut self, n: u8) -> Self {
    self.nstart = Some(n);
    self
  }

  /// Set the maximum number of times we should re-send
  /// confirmable requests before getting a response.
  ///
  /// The default value is 4 attempts
  pub fn max_con_request_retries(mut self, max_tries: crate::retry::Attempts) -> Self {
    self.max_retransmit_attempts = Some(max_tries.0);
    self
  }

  /// Set the maximum amount of time we should wait to
  /// respond to incoming multicast requests.
  ///
  /// The default value is 5 seconds.
  #[doc = rfc_7252_doc!("8.2")]
  pub fn default_leisure(mut self, default_leisure: Milliseconds<u32>) -> Self {
    self.default_leisure_millis = Some(default_leisure.0);
    self
  }

  /// Set the amount of time we should wait before resending
  /// CON requests that haven't been ACKed.
  ///
  /// The default value is `2000..=3000` milliseconds.
  pub fn ack_timeout(mut self, ack_timeout: Range<Milliseconds<u32>>) -> Self {
    self.ack_timeout_min_millis = Some(ack_timeout.start.0);
    self.ack_timeout_max_millis = Some(ack_timeout.end.0);
    self
  }
}

impl From<Config> for ConfigData {
  fn from(Config { token_seed,
                   default_leisure_millis,
                   max_retransmit_attempts,
                   nstart,
                   probing_rate_bytes_per_sec,
    ack_timeout_max_millis,
    ack_timeout_min_millis, }: Config)
          -> Self {
    ConfigData { token_seed: token_seed.unwrap_or(0),
                 default_leisure_millis: default_leisure_millis.unwrap_or(5_000),
                 max_retransmit_attempts: max_retransmit_attempts.unwrap_or(4),
                 nstart: nstart.unwrap_or(1),
                 probing_rate_bytes_per_sec: probing_rate_bytes_per_sec.unwrap_or(1_000),
        ack_timeout_max_millis: ack_timeout_max_millis.unwrap_or(3_000),
        ack_timeout_min_millis: ack_timeout_min_millis.unwrap_or(2_000), }
  }
}
