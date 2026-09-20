//! The WSJT-CB (11 m) callsign grammar.
//!
//! WSJT-CB ([`vash909/WSJT-CB`](https://github.com/vash909/WSJT-CB)) widened
//! the FT8 family to the citizens' band, where an identifier is
//! `N{1,3}L{1,2}N{1,3}` — a numeric country prefix, one or two letters, and a
//! numeric unit number — rather than the amateur `[prefix][digit][letter-suffix]`
//! shape. This is the grammar [`is_cb_callsign`] implements, and the fork's
//! decoder uses it to recognise those identifiers; on air they travel as free
//! text or a hashed non-standard call (see `sdroxide-digi`).
//!
//! It began life inside the fork's `mfsk-core` pin, offered upstream as
//! `jl1nie/mfsk-core#373`. That maintainer's call (2026-09-20) was that a
//! *dialect's* grammar is application policy rather than something the codec
//! should carry, so it lives here now. The signature is deliberately the one
//! the field-based decode hook wants —
//! `DecodeRequest::also_accept(|m| m.callsigns().all(is_cb_callsign))`, their
//! #386 — so that when that lands, only the call site changes, not this. See
//! `AGENTS.md`.

/// Check whether a string is a CB-style callsign in the WSJT-CB sense.
///
/// WSJT-CB (vash909/WSJT-CB) widened the FT8 family to the 11 m CB world,
/// where an identifier is `N{1,3}L{1,2}N{1,3}` — a numeric country prefix,
/// one or two letters, and a numeric unit number — rather than the amateur
/// `[prefix][digit][letter-suffix]` shape. Two extensions ride on top:
///
/// * a **four-digit unit number is allowed only behind a one-digit prefix**
///   (`1TT1000` passes; `26AT1000` and `111TT1000` do not), and
/// * a trailing **slash form** `N{1,3}L{1,2}/L{2}` (`999ZZ/ZZ`), whose base
///   carries no unit number of its own.
///
/// Patterns with a three-letter middle part, no numeric prefix, no numeric
/// unit, a four-digit unit behind a multi-digit prefix, or a slash that is
/// not a final `/LL` are rejected — mirroring WSJT-CB's
/// `Radio::is_cb_callsign` exactly (25-case table in the tests).
pub fn is_cb_callsign(call: &str) -> bool {
    let b = call.trim().as_bytes();
    if b.is_empty() {
        return false;
    }
    // Trailing `/LL` — the split form. Only one slash, and it must delimit
    // the final two letters: `999ZZ/ZZ` passes, `1/AT100` does not.
    if let Some(sl) = b.iter().rposition(|&c| c == b'/') {
        if b[..sl].contains(&b'/') {
            return false;
        }
        let sfx = &b[sl + 1..];
        if sfx.len() != 2 || !sfx.iter().all(|c| c.is_ascii_uppercase()) {
            return false;
        }
        return cb_prefix_digits_letters(&b[..sl]);
    }
    cb_prefix_digits_letters_digits(b)
}

/// The split-form base `N{1,3}L{1,2}`.
fn cb_prefix_digits_letters(b: &[u8]) -> bool {
    let digits = b.iter().take_while(|&&c| c.is_ascii_digit()).count();
    let letters = b[digits..].iter().take_while(|&&c| c.is_ascii_uppercase()).count();
    (1..=3).contains(&digits) && (1..=2).contains(&letters) && digits + letters == b.len()
}

/// The closed form `N{1,3}L{1,2}N{1,4}` with the four-digit-unit caveat.
fn cb_prefix_digits_letters_digits(b: &[u8]) -> bool {
    let digits = b.iter().take_while(|&&c| c.is_ascii_digit()).count();
    let letters = b[digits..].iter().take_while(|&&c| c.is_ascii_uppercase()).count();
    let units = b[digits + letters..].iter().take_while(|&&c| c.is_ascii_digit()).count();
    if !(1..=3).contains(&digits) || !(1..=2).contains(&letters) {
        return false;
    }
    if digits + letters + units != b.len() {
        return false;
    }
    if !(1..=4).contains(&units) {
        return false;
    }
    // A four-digit unit number only behind a one-digit prefix.
    units < 4 || digits == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The WSJT-CB acceptance table, mirrored exactly from
    /// vash909/WSJT-CB's README so the two validators agree case for case.
    const WSJT_CB_TABLE: &[(&str, bool)] = &[
        // (callsign, accepted by WSJT-CB)
        ("1A1", true),        // 1-digit prefix, 1 letter, 1-digit suffix
        ("1TT1", true),       // 1-digit prefix, 2 letters, 1-digit suffix
        ("1TT01", true),      // 1-digit prefix, 2 letters, 2-digit suffix
        ("1TT001", true),     // 1-digit prefix, 2 letters, 3-digit suffix
        ("1TT1000", true),    // 4-digit suffix, allowed with a 1-digit prefix
        ("1AT1000", true),    // 4-digit suffix, allowed with a 1-digit prefix
        ("11TT1", true),      // 2-digit prefix, 2 letters, 1-digit suffix
        ("111TT11", true),    // 3-digit prefix, 2 letters, 2-digit suffix
        ("111TT999", true),   // 3-digit prefix, 2 letters, 3-digit suffix
        ("26AT715", true),    // 2-digit prefix, 2 letters, 3-digit suffix
        ("999ZZ/ZZ", true),   // slash form: 3-digit prefix, 2 letters, /ZZ
        ("26AT1000", false),  // 4-digit suffix is not allowed with a 2-digit prefix
        ("111TT1000", false), // 4-digit suffix is not allowed with a 3-digit prefix
        ("99Z9999", false),   // 4-digit suffix is not allowed with a 2-digit prefix
        ("1TT10000", false),  // suffix longer than 4 digits is not allowed
        ("1TT", false),       // missing numeric suffix
        ("1AT", false),       // missing numeric suffix
        ("AT1000", false),    // missing numeric prefix
        ("AAA", false),       // letters only; numeric prefix and suffix are both missing
        ("123", false),       // digits only; the alphabetic middle part is missing
        ("12A", false),       // missing numeric suffix
        ("12ABC1", false),    // 3-letter middle part is not allowed
        ("ABC123", false),    // missing numeric prefix
        ("1/AT100", false),   // slash is only allowed as a final /LL suffix
        ("999ZZ", false),     // closed form still needs its numeric unit
    ];

    #[test]
    fn cb_callsigns_match_wsjt_cb_rejection_table() {
        for &(call, accepted) in WSJT_CB_TABLE {
            assert_eq!(is_cb_callsign(call), accepted, "is_cb_callsign({call})");
        }
    }

    #[test]
    fn surrounding_whitespace_is_ignored() {
        assert!(is_cb_callsign("  26AT715 "));
        assert!(!is_cb_callsign("   "));
        assert!(!is_cb_callsign(""));
    }
}
