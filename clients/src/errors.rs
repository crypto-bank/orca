//! Markets API clients error types.

error_chain! {
    foreign_links {
        Utf8(::std::string::FromUtf8Error) #[doc = "String unicode error."];
        Hyper(::hyper::Error) #[doc = "Hyper HTTP Client error."];
        JsonDecode(::serde_json::Error) #[doc = "JSON deserializing error."];
    }

    errors {
        Api(msg: String) {
            description("API error")
            display("API error message: `{}`", msg)
        }
    }

}
