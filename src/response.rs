use crate::error::AlipayResult as Result;
use serde::de::DeserializeOwned;
use std::io::Read;
use ureq::Response as UreqResponse;

pub struct Response(UreqResponse);

impl Response {
    pub fn new(resp: UreqResponse) -> Self {
        Response(resp)
    }

    /// The URL we ended up at. This can differ from the request url when
    /// we have followed redirects.
    pub fn get_url(&self) -> &str {
        self.0.get_url()
    }

    /// The http version: `HTTP/1.1`
    pub fn http_version(&self) -> &str {
        self.0.http_version()
    }

    /// The status as a u16: `200`
    pub fn status(&self) -> u16 {
        self.0.status()
    }

    /// The status text: `OK`
    ///
    /// The HTTP spec allows for non-utf8 status texts. This uses from_utf8_lossy to
    /// convert such lines to &str.
    pub fn status_text(&self) -> &str {
        self.0.status_text()
    }

    /// The header value for the given name, or None if not found.
    ///
    /// For historical reasons, the HTTP spec allows for header values
    /// to be encoded using encodings like iso-8859-1. Such encodings
    /// means the values are not possible to interpret as utf-8.
    ///
    /// In case the header value can't be read as utf-8, this function
    /// returns `None` (while the name is visible in [`Response::headers_names()`]).
    pub fn header(&self, name: &str) -> Option<&str> {
        self.0.header(name)
    }

    /// A list of the header names in this response.
    /// Lowercased to be uniform.
    ///
    /// It's possible for a header name to be returned by this function, and
    /// still give a `None` value. See [`Response::header()`] for an explanation
    /// as to why.
    pub fn headers_names(&self) -> Vec<String> {
        self.0.headers_names()
    }

    /// Tells if the response has the named header.
    pub fn has(&self, name: &str) -> bool {
        self.0.has(name)
    }

    /// All headers corresponding values for the give name, or empty vector.
    pub fn all(&self, name: &str) -> Vec<&str> {
        self.0.all(name)
    }

    /// The content type part of the "Content-Type" header without
    /// the charset.
    pub fn content_type(&self) -> &str {
        self.0.content_type()
    }

    /// The character set part of the "Content-Type".
    pub fn charset(&self) -> &str {
        self.0.charset()
    }

    /// Turn this response into a `impl Read` of the body.
    ///
    /// 1. If `Transfer-Encoding: chunked`, the returned reader will unchunk it
    ///    and any `Content-Length` header is ignored.
    /// 2. If `Content-Length` is set, the returned reader is limited to this byte
    ///    length regardless of how many bytes the server sends.
    /// 3. If no length header, the reader is until server stream end.
    ///
    /// Note: If you use `read_to_end()` on the resulting reader, a malicious
    /// server might return enough bytes to exhaust available memory. If you're
    /// making requests to untrusted servers, you should use `.take()` to
    /// limit the response bytes read.
    pub fn into_reader(self) -> Box<dyn Read + Send + Sync + 'static> {
        self.0.into_reader()
    }

    /// Turn this response into a String of the response body. By default uses `utf-8`,
    /// but can work with charset, see below.
    ///
    /// This is potentially memory inefficient for large bodies since the
    /// implementation first reads the reader to end into a `Vec<u8>` and then
    /// attempts to decode it using the charset.
    ///
    /// If the response is larger than 10 megabytes, this will return an error.
    pub fn into_string(self) -> Result<String> {
        Ok(self.0.into_string()?)
    }

    /// Read the body of this response into a serde_json::Value, or any other type that
    /// implements the [serde::Deserialize] trait.
    ///
    /// You must use either a type annotation as shown below (`message: Message`), or the
    /// [turbofish operator] (`::<Type>`) so Rust knows what type you are trying to read.
    pub fn into_json<T: DeserializeOwned>(self) -> Result<T> {
        Ok(self.0.into_json()?)
    }
}
