
error_chain! {
    foreign_links {
        IoError(::std::io::Error);
        ParseIntError(::std::num::ParseIntError);
        ParseFloatError(::std::num::ParseFloatError);
        DeserializeError(::serde_json::error::Error);
        WebSocketError(::websocket::result::WebSocketError);
    }

    errors {
        UnexpectedMessageType(t: String) {
            description("unexpected message type")
            display("unexpected message type: {}", t)
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

pub type Future<T> = ::futures::Future<Item=T, Error=Error>;
