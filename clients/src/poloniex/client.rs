//! Poloniex market orderbook API client.

use std::str::FromStr;
use std::collections::HashMap;

use chrono::UTC;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

use data_encoding::HEXLOWER;

use futures::Stream;
use futures::future::{self, Future};

use super::super::{Auth, Response};
use super::super::errors::*;
use super::super::util::urlencode;

/// Poloniex market API client.
pub struct Client {
    http: ::hyper::Client<::hyper_tls::HttpsConnector>,
    auth: Auth,
}

impl Client {

    /// Creates new poloniex client.
    pub fn new(http: ::hyper::Client<::hyper_tls::HttpsConnector>, auth: Auth) -> Self {
        Client {
            http: http,
            auth: auth,
        }
    }

    /// 
    pub fn balances(&mut self) -> Box<Response<HashMap<String, String>>> {
        let params = HashMap::new();
        self.request_auth("returnBalances", &params)
    }

    /// Sends authenticated request to poloniex api.
    fn request_auth(&mut self, command: &str, params: &HashMap<&str, &str>)
        -> Box<Response<HashMap<String, String>>>
    {
        // Current timestamp required for signing
        let timestamp = (UTC::now().timestamp() * 1000).to_string();
        // Create clone of params and set command and nonce
        let mut params = params.clone();
        params.insert("command", command);
        params.insert("nonce", &timestamp);
        // Encode body
        let body = urlencode(&params);
        // Sign body
        let mut hmac = Hmac::new(Sha512::new(), self.auth.secret.as_bytes());
        hmac.input(body.as_bytes());
        // encode as hex
        let sign = HEXLOWER.encode(hmac.result().code());
        // create request
        let uri = ::hyper::Uri::from_str("https://poloniex.com/tradingApi").unwrap();
        let mut request = ::hyper::client::Request::new(::hyper::Method::Post, uri);
        request.set_body(body);
        request.headers_mut().set(headers::Key(self.auth.key.clone()));
        request.headers_mut().set(headers::Sign(sign));
        request.headers_mut().set(headers::Content("application/x-www-form-urlencoded".to_owned()));
        // send signed request
        let response = self.http.request(request)
            .and_then(|resp| {
                resp.body().fold(Vec::new(), |mut vec, chunk| {
                    vec.extend(&chunk[..]);
                    future::ok::<_, ::hyper::Error>(vec)
                })
            })
            .map_err(|e| Error::from_kind(ErrorKind::Hyper(e)));
        // unmarshal response
        let response = response.map(|chunks| {
            let body = String::from_utf8(chunks).unwrap();
            let value: HashMap<String, String> = ::serde_json::from_str(&body).unwrap();
            Box::new(value)
        }).and_then(|value| {
            let error_key = "error";
            let res = if value.contains_key(error_key) {
                let msg = value.get(error_key).unwrap();
                Err(Error::from_kind(ErrorKind::Api(msg.to_owned())))
            } else {
                Ok(value)
            };
            future::result(res)
        });
        Box::new(response)
    }

    /// Sends unauthenticated request to poloniex api.
    fn request_public(&mut self, command: &str, params: &HashMap<&str, &str>)
        -> Box<Response<HashMap<String, String>>>
    {
        let params = urlencode(params);
        let uri = ::hyper::Uri::from_str(&format!("https://poloniex.com/public?command={}&{}",
            command, &params)).unwrap();
        let response = self.http.get(uri).and_then(|resp| {
            resp.body().fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, ::hyper::Error>(v)
            })
        }).map(|chunks| {
            let body = String::from_utf8(chunks).unwrap();
            let value: HashMap<String, String> = ::serde_json::from_str(&body).unwrap();
            Box::new(value)
        }).map_err(|e| Error::from_kind(ErrorKind::Hyper(e)));
        Box::new(response)
    }
}

mod headers {
    header! {
        #[doc(hidden)]
        (Key, "Key") => [String]
    }
    header! {
        #[doc(hidden)]
        (Sign, "Sign") => [String]
    }
    header! {
        #[doc(hidden)]
        (Content, "Content-Type") => [String]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let mut core = ::tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        let http = ::hyper::Client::configure()
            .connector(::hyper_tls::HttpsConnector::new(1, &handle))
            .build(&handle);
        let auth = Auth {
            key: "test".to_owned(),
            secret: "test".to_owned(),
        };
        let mut client = Client::new(http, auth);

        core.run(client.balances().then(|res| {
            match res.unwrap_err().kind() {
                &ErrorKind::Api(ref msg) => {
                    assert_eq!(msg, "Invalid API key/secret pair.");
                    Ok::<_, ()>(())
                },
                _ => panic!("unexpected error"),
            }
        })).unwrap();
    }
}
