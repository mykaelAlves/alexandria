/// Based on RFC 9457
///
/// # Example
///
/// {
///
///  "type": "https://example.com/probs/out-of-credit",
///
///  "title": "You do not have enough credit.",
///
///  "detail": "Your current balance is 30, but that costs 50.",
///
///  "instance": "/account/12345/msgs/abc",
///
///  "balance": 30,
///
///  "accounts": \[ "/account/12345",
///               "/account/67890" \]
///
/// }
pub struct ProblemDetailJsonObject {
    /// The "type" member is a JSON string containing a URI reference
    /// that identifies the problem type. Consumers MUST use the "type" URI
    /// (after resolution, if necessary) as the problem type's primary
    /// identifier.
    ///
    /// When this member is not present, its value is assumed to be
    /// "about:blank".
    ///
    /// If the type URI is a locator (e.g., those with an "http" or "https"
    /// scheme), dereferencing it SHOULD provide human-readable documentation
    /// for the problem type (e.g., using HTML). However, consumers
    /// SHOULD NOT automatically dereference the type URI, unless they do so
    /// when providing information to developers (e.g., when a debugging tool
    /// is in use).
    ///
    /// When "type" contains a relative URI, it is resolved relative to the
    /// document's base URI. However, using relative
    /// URIs can cause confusion, and they might not be handled correctly by
    /// all implementations.
    type_: String,
    status: String,
    title: String,
    details: String,
    instance: String,
}
