//! Core error kinds.

error_chain! {
    foreign_links {
        // Parsers
        ParseIntError(::std::num::ParseIntError);
        ParseFloatError(::std::num::ParseFloatError);

        // Streams
        WebSocketError(::websocket::result::WebSocketError);
        SendEventsError(::futures::sync::mpsc::SendError<::streams::Events>);
        SendCommandError(::std::sync::mpsc::TrySendError<::streams::Command>);

        // Serialization
        JsonError(::serde_json::error::Error);
        ProtobufError(::protobuf::error::ProtobufError);
    }

    errors {
        // Currency
        UnknownCurrency(symbol: String) {
            description("unknown currency symbol")
            display("unknown currency symbol: {}", symbol)
        }
        // Currency pair
        InvalidPair(pair: String) {
            description("unknown currency pair")
            display("unknown currency pair: {}", pair)
        }
        // Streams
        UnknownEventType(t: String) {
            description("unexpected event type")
            display("unexpected event type: {}", t)
        }
        // Markets
        InvalidOrderKind(k: i64) {
            description("invalid order kind")
            display("invalid order kind: {}", k)
        }
        // Utils
        EmptyOption {
            description("unwrapped empty option")
        }
    }
}

/// Future with core error type.
pub type Future<T> = ::futures::Future<Item = T, Error = Error>;

/// Boxed future with core error type.
pub type BoxFuture<T> = Box<Future<T>>;
