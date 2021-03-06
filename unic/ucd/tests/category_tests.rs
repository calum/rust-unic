// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(test)]


#[macro_use]
extern crate matches;

extern crate unic_ucd;
extern crate unic_utils;


use unic_ucd::bidi::BidiClass as BC;
use unic_ucd::normal::is_combining_mark;
use unic_ucd::category::GeneralCategory as GC;
use unic_utils::iter_all_chars;

/// <http://www.unicode.org/reports/tr9/#EN>
#[test]
fn test_bidi_en_against_gen_cat() {
    for cp in iter_all_chars() {
        if BC::of(cp) == BC::EuropeanNumber {
            assert!(GC::of(cp).is_number());
        }
    }
}

/// <http://www.unicode.org/reports/tr9/#ES>
#[test]
fn test_bidi_es_against_gen_cat() {
    for cp in iter_all_chars() {
        if BC::of(cp) == BC::EuropeanSeparator {
            assert!(matches!(GC::of(cp) , GC::MathSymbol | GC::DashPunctuation));
        }
    }
}

/// <http://www.unicode.org/reports/tr9/#ET>
#[test]
fn test_bidi_et_against_gen_cat() {
    for cp in iter_all_chars() {
        if BC::of(cp) == BC::EuropeanTerminator {
            assert!(
                GC::of(cp).is_symbol()          ||
                matches!(GC::of(cp) , GC::Unassigned | GC::OtherPunctuation)
            );
        }
    }
}

/// <http://www.unicode.org/reports/tr9/#AN>
#[test]
fn test_bidi_an_against_gen_cat() {
    for cp in iter_all_chars() {
        if BC::of(cp) == BC::ArabicNumber {
            assert!(
                matches!(GC::of(cp) , GC::Format | GC::OtherNumber | GC::OtherPunctuation | GC::DecimalNumber)
            );
        }
    }
}

/// <http://www.unicode.org/reports/tr9/#CS>
#[test]
fn test_bidi_cs_against_gen_cat() {
    for cp in iter_all_chars() {
        if BC::of(cp) == BC::CommonSeparator {
            assert!(matches!(GC::of(cp) , GC::OtherPunctuation | GC::SpaceSeparator | GC::MathSymbol));
        }
    }
}

/// `Bidi_Class=NSM := General_Category in { Mn (Nonspacing_Mark), Me (Enclosing_Mark) }`
///
/// <http://www.unicode.org/reports/tr9/#NSM>
#[test]
fn test_bidi_nsm_against_gen_cat() {
    // Every NSM must be a GC=Mark
    for cp in iter_all_chars() {
        if BC::of(cp) == BC::NonspacingMark {
            assert!(is_combining_mark(cp));
        }
    }

    // Every GC!=Mark must not be an NSM
    for cp in iter_all_chars() {
        if !is_combining_mark(cp) {
            assert_ne!(BC::of(cp), BC::NonspacingMark);
        }
    }
}

/// <http://www.unicode.org/reports/tr9/#BN>
#[test]
fn test_bidi_bn_against_gen_cat() {
    for cp in iter_all_chars() {
        if BC::of(cp) == BC::BoundaryNeutral {
            assert!(GC::of(cp).is_other());
        }
    }
}

/// <http://www.unicode.org/reports/tr9/#B>
#[test]
fn test_bidi_b_against_gen_cat() {
    for cp in iter_all_chars() {
        if BC::of(cp) == BC::ParagraphSeparator {
            assert!(matches!(GC::of(cp) , GC::Control | GC::ParagraphSeparator));
        }
    }
}

/// <http://www.unicode.org/reports/tr9/#S>
#[test]
fn test_bidi_s_against_gen_cat() {
    for cp in iter_all_chars() {
        if BC::of(cp) == BC::SegmentSeparator {
            assert!(matches!(GC::of(cp) , GC::Control));
        }
    }
}

/// <http://www.unicode.org/reports/tr9/#WS>
#[test]
fn test_bidi_ws_against_gen_cat() {
    for cp in iter_all_chars() {
        if BC::of(cp) == BC::WhiteSpace {
            assert!(matches!(GC::of(cp) , GC::Control | GC::SpaceSeparator | GC::LineSeparator));
        }
    }
}
