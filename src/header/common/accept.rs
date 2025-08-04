use mime::Mime;

use header::{QualityItem, qitem};

header! {
    /// `Accept` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.3.2)
    ///
    /// The `Accept` header field can be used by user agents to specify
    /// response media types that are acceptable.  Accept header fields can
    /// be used to indicate that the request is specifically limited to a
    /// small set of desired types, as in the case of a request for an
    /// in-line image
    ///
    /// # ABNF
    /// ```plain
    /// Accept = #( media-range [ accept-params ] )
    ///
    /// media-range    = ( "*/*"
    ///                  / ( type "/" "*" )
    ///                  / ( type "/" subtype )
    ///                  ) *( OWS ";" OWS parameter )
    /// accept-params  = weight *( accept-ext )
    /// accept-ext = OWS ";" OWS token [ "=" ( token / quoted-string ) ]
    /// ```
    ///
    /// # Example values
    /// * `audio/*; q=0.2, audio/basic` (`*` value won't parse correctly)
    /// * `text/plain; q=0.5, text/html, text/x-dvi; q=0.8, text/x-c`
    ///
    /// # Examples
    /// ```
    /// use hyper::header::{Headers, Accept, qitem};
    /// use hyper::mime::Mime;
    ///
    /// let mut headers = Headers::new();
    ///
    /// headers.set(
    ///     Accept(vec![
    ///         qitem(mime::TEXT_HTML),
    ///     ])
    /// );
    /// ```
    /// ```
    /// use hyper::header::{Headers, Accept, qitem};
    /// use hyper::mime::Mime;
    /// use std::str::FromStr;
    ///
    /// let mut headers = Headers::new();
    /// headers.set(
    ///     Accept(vec![
    ///         qitem(Mime::from_str("application/json; charset=utf-8").unwrap())
    ///     ])
    /// );
    /// ```
    /// ```
    /// use hyper::header::{Headers, Accept, QualityItem, Quality, qitem};
    /// use hyper::mime::Mime;
    /// use std::str::FromStr;
    ///
    /// let mut headers = Headers::new();
    ///
    /// headers.set(
    ///     Accept(vec![
    ///         qitem(mime::TEXT_HTML),
    ///         qitem(Mime::from_str("application/xhtml+xml").unwrap()),
    ///         QualityItem::new(Mime::from_str("application/xml").unwrap(), Quality(900)),
    ///         qitem(Mime::from_str("image/webp").unwrap()),
    ///         QualityItem::new(mime::STAR_STAR, Quality(800))
    ///     ])
    /// );
    /// ```
    ///
    /// # Notes
    /// * Using always Mime types to represent `media-range` differs from the ABNF.
    /// * **FIXME**: `accept-ext` is not supported.
    (Accept, "Accept") => (QualityItem<Mime>)+

    test_accept {
        use std::str::FromStr;
        // Tests from the RFC
        // FIXME: Test fails, first value containing a "*" fails to parse
        // test_header!(
        //    test1,
        //    vec![b"audio/*; q=0.2, audio/basic"],
        //    Some(HeaderField(vec![
        //        QualityItem::new(Mime(TopLevel::Audio, SubLevel::Star, vec![]), Quality(200)),
        //        qitem(Mime(TopLevel::Audio, SubLevel::Ext("basic".to_owned()), vec![])),
        //        ])));
        test_header!(
            test2,
            vec![b"text/plain; q=0.5, text/html, text/x-dvi; q=0.8, text/x-c"],
            Some(HeaderField(vec![
                QualityItem::new(mime::TEXT_PLAIN, Quality(500)),
                qitem(mime::TEXT_HTML),
                QualityItem::new(Mime::from_str("text/x-dvi").unwrap(), Quality(800)),
                qitem(Mime::from_str("text/x-c").unwrap()),
                ])));
        // Custom tests
        test_header!(
            test3,
            vec![b"text/plain; charset=utf-8"],
            Some(Accept(vec![qitem(mime::TEXT_PLAIN_UTF_8),])));
        test_header!(
            test4,
            vec![b"text/plain; charset=utf-8; q=0.5"],
            Some(Accept(vec![
                QualityItem::new(mime::TEXT_PLAIN_UTF_8, Quality(500)),
            ])));
    }
}

impl Accept {
    /// A constructor to easily create `Accept: */*`.
    pub fn star() -> Accept {
        Accept(vec![qitem(mime::STAR_STAR)])
    }

    /// A constructor to easily create `Accept: application/json`.
    pub fn json() -> Accept {
        Accept(vec![qitem(mime::APPLICATION_JSON)])
    }

    /// A constructor to easily create `Accept: text/*`.
    pub fn text() -> Accept {
        Accept(vec![qitem(mime::TEXT_STAR)])
    }

    /// A constructor to easily create `Accept: image/*`.
    pub fn image() -> Accept {
        Accept(vec![qitem(mime::IMAGE_STAR)])
    }
}


bench_header!(bench, Accept, { vec![b"text/plain; q=0.5, text/html".to_vec()] });
