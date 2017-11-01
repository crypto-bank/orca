
error_chain! {
    foreign_links {
        IoError(::std::io::Error);
        ParseIntError(::std::num::ParseIntError);
        ParseFloatError(::std::num::ParseFloatError);
        DeserializeError(::serde_json::error::Error);
        WebSocketError(::websocket::result::WebSocketError);
        SendEventsError(::futures::sync::mpsc::SendError<::core::streams::Events>);
    }

    errors {
        UnexpectedEventType(t: String) {
            description("unexpected event type")
            display("unexpected event type: {}", t)
        }
        InvalidMarket(market: String) {
            description("invalid market")
            display("invalid market: {}", market)
        }
        InvalidCurrencyPair(pair: String) {
            description("invalid currency pair")
            display("invalid currency pair: {}", pair)
        }
        InvalidOrderKind(k: i64) {
            description("invalid order kind")
            display("invalid order kind: {}", k)
        }
        ValueNotFound {
            description("value not found")
        }
    }
}

// above are placed here for `use core::errors::*;`

/// Future with core error type.
pub type Future<T> = ::futures::Future<Item=T, Error=Error>;

/// Boxed future with core error type.
pub type BoxFuture<T> = Box<Future<T>>;
