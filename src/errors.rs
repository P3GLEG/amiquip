use failure::{Backtrace, Context, Fail};
use std::sync::Arc;
use std::{fmt, result};

/// A type alias for handling errors throughout amiquip.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur from amiquip.
#[derive(Clone, Debug)]
pub struct Error {
    ctx: Arc<Context<ErrorKind>>,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.ctx.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

/// Specific error cases returned by amiquip.
#[derive(Clone, Debug, PartialEq, Fail)]
pub enum ErrorKind {
    /// The underlying socket was closed.
    #[fail(display = "underlying socket closed unexpectedly")]
    UnexpectedSocketClose,

    /// We received data that could not be parsed as an AMQP frame.
    #[fail(display = "received malformed data - expected AMQP frame")]
    MalformedFrame,

    /// An I/O error occurred; the underlying cause will be an `io::Error`.
    #[fail(display = "I/O error")]
    Io,

    /// The TLS handshake failed.
    #[cfg(feature = "native-tls")]
    #[fail(display = "TLS handshake failed")]
    TlsHandshake,

    /// The server does not support the requested auth mechanism.
    #[fail(display = "requested auth mechanism unavailable (available = {})", _0)]
    UnsupportedAuthMechanism(String),

    /// The server does not support the requested locale.
    #[fail(display = "requested locale unavailable (available = {})", _0)]
    UnsupportedLocale(String),

    /// The requested frame size is smaller than the minimum required by AMQP.
    #[fail(display = "requested frame max is too small (min = {})", _0)]
    FrameMaxTooSmall(u32),

    /// A timeout has occurred while waiting for poll events. This error will fire
    /// if a connection is created with a non-`None`
    /// [poll_timeout](struct.ConnectionTuning.html#structfield.poll_timeout) and that period of
    /// time passes with no events waking up the I/O thread. Note that this timeout does not
    /// necessarily indicate communication with the AMQP server, as internal channel messages also
    /// wake up the I/O thread.
    #[fail(display = "timeout occurred while waiting for poll events")]
    PollTimeout,

    /// The server requested a Secure/Secure-Ok exchange, which are currently unsupported.
    #[fail(display = "SASL secure/secure-ok exchanges are not supported")]
    SaslSecureNotSupported,

    /// The supplied authentication credentials were not accepted by the server.
    #[fail(display = "invalid credentials")]
    InvalidCredentials,

    /// The server missed too many successive heartbeats.
    #[fail(display = "missed heartbeats from server")]
    MissedServerHeartbeats,

    /// The server closed the connection with the given reply code and text.
    #[fail(display = "server closed connection (code={} message={})", _0, _1)]
    ServerClosedConnection(u16, String),

    /// The client closed the connection.
    #[fail(display = "client closed connection")]
    ClientClosedConnection,

    /// The server closed the given channel with the given reply code and text.
    #[fail(display = "server closed channel {} (code={}, message={})", _0, _1, _2)]
    ServerClosedChannel(u16, u16, String),

    /// The client closed the channel.
    #[fail(display = "channel has been closed")]
    ClientClosedChannel,

    /// The I/O loop attempted to send a message to a caller that did not exist. This
    /// indicates either a bug in amiquip or a connection that is in a bad state and in the process
    /// of tearing down.
    #[fail(display = "i/o loop thread tried to communicate with a nonexistent client")]
    EventLoopClientDropped,

    /// The I/O loop has dropped the sending side of a channel, typically because it has exited due
    /// to another error.
    #[fail(display = "i/o loop dropped sending side of a channel")]
    EventLoopDropped,

    /// We received a valid AMQP frame but not one we expected; e.g., receiving an incorrect
    /// response to an AMQP method call.
    #[fail(display = "AMQP protocol error - received unexpected frame")]
    FrameUnexpected,

    /// Forking the I/O thread failed.
    #[fail(display = "fork failed")]
    ForkFailed,

    /// No more channels can be opened because there are already
    /// [`channel_max`](struct.ConnectionOptions.html#method.channel_max) channels open.
    #[fail(display = "no more channel ids are available")]
    ExhaustedChannelIds,

    /// An explicit channel ID was requested, but that channel is unavailable for use (e.g.,
    /// because there is another open channel with the same ID).
    #[fail(display = "requested channel id {} is unavailable", _0)]
    UnavailableChannelId(u16),

    /// The client sent an AMQP exception to the server and closed the connection.
    #[fail(display = "internal client exception - received unhandled frames from server")]
    ClientException,

    /// The server sent frames for a channel ID we don't know about.
    #[fail(display = "received message for nonexistent channel {}", _0)]
    ReceivedFrameWithBogusChannelId(u16),

    /// The I/O thread panicked.
    #[fail(display = "I/O thread died unexpectedly: {}", _0)]
    IoThreadPanic(String),

    /// The server sent us a consumer tag that is equal to another consumer tag we already have on
    /// the same channel.
    #[fail(
        display = "server sent duplicate consumer tag for channel {}: {}",
        _0, _1
    )]
    DuplicateConsumerTag(u16, String),

    /// The server sent us a [`Delivery`](struct.Delivery.html) for a channel we don't know about.
    #[fail(
        display = "received delivery with unknown consumer tag for channel {}: {}",
        _0, _1
    )]
    UnknownConsumerTag(u16, String),

    #[doc(hidden)]
    #[fail(display = "invalid error case")]
    __Nonexhaustive,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { ctx: Arc::new(ctx) }
    }
}
