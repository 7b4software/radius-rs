// Code generated by machine generator; DO NOT EDIT.

//! Utility for rfc3576 packet.
//!
//! This module handles the packet according to the following definition:
//! ```text
//! //! # -*- text -*-
//! # Copyright (C) 2020 The FreeRADIUS Server project and contributors
//! # This work is licensed under CC-BY version 4.0 https://creativecommons.org/licenses/by/4.0
//! # Version $Id$
//! #
//! #    Attributes and values defined in RFC 3576.
//! #    http://www.ietf.org/rfc/rfc3576.txt
//! #
//! #    $Id$
//! #
//! ATTRIBUTE    Error-Cause                101    integer
//!
//! #    Service Types
//!
//! VALUE    Service-Type            Authorize-Only        17
//!
//! #    Error causes
//!
//! VALUE    Error-Cause            Residual-Context-Removed    201
//! VALUE    Error-Cause            Invalid-EAP-Packet    202
//! VALUE    Error-Cause            Unsupported-Attribute    401
//! VALUE    Error-Cause            Missing-Attribute    402
//! VALUE    Error-Cause            NAS-Identification-Mismatch    403
//! VALUE    Error-Cause            Invalid-Request        404
//! VALUE    Error-Cause            Unsupported-Service    405
//! VALUE    Error-Cause            Unsupported-Extension    406
//! VALUE    Error-Cause            Administratively-Prohibited    501
//! VALUE    Error-Cause            Proxy-Request-Not-Routable    502
//! VALUE    Error-Cause            Session-Context-Not-Found    503
//! VALUE    Error-Cause            Session-Context-Not-Removable    504
//! VALUE    Error-Cause            Proxy-Processing-Error    505
//! VALUE    Error-Cause            Resources-Unavailable    506
//! VALUE    Error-Cause            Request-Initiated    507
//! ```

use crate::core::avp::{AVPError, AVPType, AVP};
use crate::core::packet::Packet;

use crate::core::rfc2865;

pub const ERROR_CAUSE_TYPE: AVPType = 101;
/// Delete all of `error_cause` values from a packet.
pub fn delete_error_cause(packet: &mut Packet) {
    packet.delete(ERROR_CAUSE_TYPE);
}
/// Add `error_cause` value-defined integer value to a packet.
pub fn add_error_cause(packet: &mut Packet, value: ErrorCause) {
    packet.add(AVP::from_u32(ERROR_CAUSE_TYPE, value));
}
/// Lookup a `error_cause` value-defined integer value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `error_cause`, it returns `None`.
pub fn lookup_error_cause(packet: &Packet) -> Option<Result<ErrorCause, AVPError>> {
    packet
        .lookup(ERROR_CAUSE_TYPE)
        .map(|v| Ok(v.encode_u32()? as ErrorCause))
}
/// Lookup all of the `error_cause` value-defined integer value from a packet.
/// # Errors
/// `AVPError`
pub fn lookup_all_error_cause(packet: &Packet) -> Result<Vec<ErrorCause>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(ERROR_CAUSE_TYPE) {
        vec.push(avp.encode_u32()? as ErrorCause);
    }
    Ok(vec)
}

pub type ErrorCause = u32;
pub const ERROR_CAUSE_RESIDUAL_CONTEXT_REMOVED: ErrorCause = 201;
pub const ERROR_CAUSE_INVALID_EAP_PACKET: ErrorCause = 202;
pub const ERROR_CAUSE_UNSUPPORTED_ATTRIBUTE: ErrorCause = 401;
pub const ERROR_CAUSE_MISSING_ATTRIBUTE: ErrorCause = 402;
pub const ERROR_CAUSE_NAS_IDENTIFICATION_MISMATCH: ErrorCause = 403;
pub const ERROR_CAUSE_INVALID_REQUEST: ErrorCause = 404;
pub const ERROR_CAUSE_UNSUPPORTED_SERVICE: ErrorCause = 405;
pub const ERROR_CAUSE_UNSUPPORTED_EXTENSION: ErrorCause = 406;
pub const ERROR_CAUSE_ADMINISTRATIVELY_PROHIBITED: ErrorCause = 501;
pub const ERROR_CAUSE_PROXY_REQUEST_NOT_ROUTABLE: ErrorCause = 502;
pub const ERROR_CAUSE_SESSION_CONTEXT_NOT_FOUND: ErrorCause = 503;
pub const ERROR_CAUSE_SESSION_CONTEXT_NOT_REMOVABLE: ErrorCause = 504;
pub const ERROR_CAUSE_PROXY_PROCESSING_ERROR: ErrorCause = 505;
pub const ERROR_CAUSE_RESOURCES_UNAVAILABLE: ErrorCause = 506;
pub const ERROR_CAUSE_REQUEST_INITIATED: ErrorCause = 507;

pub const SERVICE_TYPE_AUTHORIZE_ONLY: rfc2865::ServiceType = 17;
