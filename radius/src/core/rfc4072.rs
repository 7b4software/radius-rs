// Code generated by machine generator; DO NOT EDIT.

//! Utility for rfc4072 packet.
//!
//! This module handles the packet according to the following definition:
//! ```text
//! //! # -*- text -*-
//! # Copyright (C) 2020 The FreeRADIUS Server project and contributors
//! # This work is licensed under CC-BY version 4.0 https://creativecommons.org/licenses/by/4.0
//! # Version $Id$
//! #
//! #    Attributes and values defined in RFC 4072
//! #    http://www.ietf.org/rfc/rfc4072.txt
//! #
//! #    $Id$
//! #
//!
//! ATTRIBUTE    EAP-Key-Name                102    octets
//! ```

use crate::core::avp::{AVPType, AVP};
use crate::core::packet::Packet;

pub const EAP_KEY_NAME_TYPE: AVPType = 102;
/// Delete all of `eap_key_name` values from a packet.
pub fn delete_eap_key_name(packet: &mut Packet) {
    packet.delete(EAP_KEY_NAME_TYPE);
}
/// Add `eap_key_name` octets value to a packet.
pub fn add_eap_key_name(packet: &mut Packet, value: &[u8]) {
    packet.add(AVP::from_bytes(EAP_KEY_NAME_TYPE, value));
}
/// Lookup a `eap_key_name` octets value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `eap_key_name`, it returns `None`.
pub fn lookup_eap_key_name(packet: &Packet) -> Option<Vec<u8>> {
    packet.lookup(EAP_KEY_NAME_TYPE).map(AVP::encode_bytes)
}
/// Lookup all of the `eap_key_name` octets value from a packet.
/// # Errors
/// `AVPError`
pub fn lookup_all_eap_key_name(packet: &Packet) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(EAP_KEY_NAME_TYPE) {
        vec.push(avp.encode_bytes());
    }
    vec
}
