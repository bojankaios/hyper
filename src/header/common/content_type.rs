use mime::Mime;

header! {
    /// `Content-Type` header, defined in
    /// [RFC7231](http://tools.ietf.org/html/rfc7231#section-3.1.1.5)
    ///
    /// The `Content-Type` header field indicates the media type of the
    /// associated representation: either the representation enclosed in the
    /// message payload or the selected representation, as determined by the
    /// message semantics.  The indicated media type defines both the data
    /// format and how that data is intended to be processed by a recipient,
    /// within the scope of the received message semantics, after any content
    /// codings indicated by Content-Encoding are decoded.
    ///
    /// # ABNF
    /// ```plain
    /// Content-Type = media-type
    /// ```
    ///
    /// # Example values
    /// * `text/html; charset=ISO-8859-4`
    ///
    /// # Examples
    /// ```
    /// use hyper::header::{Headers, ContentType};
    /// use hyper::mime::Mime;
    ///
    /// let mut headers = Headers::new();
    ///
    /// headers.set(
    ///     ContentType(mime::TEXT_HTML)
    /// );
    /// ```
    /// ```
    /// use hyper::header::{Headers, ContentType};
    /// use hyper::mime::Mime;
    /// use std::str::FromStr;
    ///
    /// let mut headers = Headers::new();
    ///
    /// headers.set(
    ///     ContentType(Mime::from_str("application/json; charset=utf-8").unwrap())
    /// );
    /// ```
    (ContentType, "Content-Type") => [Mime]

    test_content_type {
        use std::str::FromStr;
        test_header!(
            test1,
            // FIXME: Should be b"text/html; charset=ISO-8859-4" but mime crate lowercases
            // the whole value so parsing and formatting the value gives a different result
            vec![b"text/html; charset=iso-8859-4"],
            Some(HeaderField(Mime::from_str("text/html; charset=iso-8859-4").unwrap())));
    }
}

impl ContentType {
    /// A constructor  to easily create a `Content-Type: application/json` header.
    #[inline]
    pub fn json() -> ContentType {
        ContentType(mime::APPLICATION_JSON)
    }

    /// A constructor  to easily create a `Content-Type: text/plain; charset=utf-8` header.
    #[inline]
    pub fn plaintext() -> ContentType {
        ContentType(mime::TEXT_PLAIN_UTF_8)
    }

    /// A constructor  to easily create a `Content-Type: text/html; charset=utf-8` header.
    #[inline]
    pub fn html() -> ContentType {
        ContentType(mime::TEXT_HTML_UTF_8)
    }

    /// A constructor  to easily create a `Content-Type: application/www-form-url-encoded` header.
    #[inline]
    pub fn form_url_encoded() -> ContentType {
        ContentType(mime::APPLICATION_WWW_FORM_URLENCODED)
    }
    /// A constructor  to easily create a `Content-Type: image/jpeg` header.
    #[inline]
    pub fn jpeg() -> ContentType {
        ContentType(mime::IMAGE_JPEG)
    }

    /// A constructor  to easily create a `Content-Type: image/png` header.
    #[inline]
    pub fn png() -> ContentType {
        ContentType(mime::IMAGE_PNG)
    }
}

bench_header!(bench, ContentType, { vec![b"application/json; charset=utf-8".to_vec()] });
