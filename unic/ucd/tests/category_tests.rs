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


extern crate unic_ucd;
extern crate unic_utils;


use unic_ucd::bidi::BidiClass;
use unic_ucd::normal::is_combining_mark;
use unic_ucd::category::GeneralCategory as GC;
use unic_utils::iter_all_chars;

/// `normal::is_combining_mark` and `GeneralCategory::is_mark()` are expected to return
/// the same results.
#[test]
fn test_gen_cat_against_normal() {
    for cp in iter_all_chars() {
        // Every General_Category mark must be a combining mark
        if GC::of(cp).is_mark() {
            assert!(is_combining_mark(cp));
        }

        // Every combining mark must be a General_Category mark
        if is_combining_mark(cp) {
            assert!(GC::of(cp).is_mark());
        }
    }
}

/// `Bidi_Class=EN := General_Category in { N (Number) }`
///
/// <http://www.unicode.org/reports/tr9/#EN>
#[test]
fn test_bidi_en_against_gen_cat() {
    // Every BC::EuropeanNumber must satisfy GC::is_number()
    for cp in iter_all_chars() {
        if BidiClass::of(cp) == BidiClass::EuropeanNumber {
            assert!(GC::of(cp).is_number());
        }
    }
}

/// `Bidi_Class=ES := General_Category in { Sm (MathSymbol), Pd (DashPunctuation) }`
///
/// <http://www.unicode.org/reports/tr9/#ES>
#[test]
fn test_bidi_es_against_gen_cat() {
    // Every BC::EuropeanSeparator must be a GC::MathSymbol, GC::DashPunctuation
    for cp in iter_all_chars() {
        if BidiClass::of(cp) == BidiClass::EuropeanSeparator {
            assert!(
                GC::of(cp) == GC::MathSymbol ||
                GC::of(cp) == GC::DashPunctuation
            );
        }
    }
}

/// `Bidi_Class=ET := General_Category in { S (Symbol), Cn (Unassigned),
///                                         Po (OtherPunctuation) }`
///
/// <http://www.unicode.org/reports/tr9/#ET>
#[test]
fn test_bidi_et_against_gen_cat() {
    // Every BC::EuropeanTerminator must satisfy GC::is_symbol() or
    // be GC::Unassigned or GC::OtherPunctuation
    for cp in iter_all_chars() {
        if BidiClass::of(cp) == BidiClass::EuropeanTerminator {
            assert!(
                GC::of(cp).is_symbol()          ||
                GC::of(cp) == GC::Unassigned    ||
                GC::of(cp) == GC::OtherPunctuation
            );
        }
    }
}

/// `Bidi_Class=AN := General_Category in { Cf (Format), No (OtherNumber),
///                                         Po (OtherPunctuation), Nd (DecimalNumber) }`
///
/// <http://www.unicode.org/reports/tr9/#AN>
#[test]
fn test_bidi_an_against_gen_cat() {
    // Every BC::ArabicNumber must be a GC::Format, GC::OtherNumber, GC::OtherPunctuation,
    // or GC::DecimalNumber
    for cp in iter_all_chars() {
        if BidiClass::of(cp) == BidiClass::ArabicNumber {
            assert!(
                GC::of(cp) == GC::Format            ||
                GC::of(cp) == GC::OtherNumber       ||
                GC::of(cp) == GC::OtherPunctuation  ||
                GC::of(cp) == GC::DecimalNumber
            );
        }
    }
}

/// `Bidi_Class=CS := General_Category in { Po (OtherPunctuation), Zs (SpaceSeparator),
///                                         Sm (MathSymbol) }`
///
/// <http://www.unicode.org/reports/tr9/#CS>
#[test]
fn test_bidi_cs_against_gen_cat() {
    // Every BC::CommonSeparator must be a GC::OtherPunctuation, GC::SpaceSeparator,
    // or GC::MathSymbol
    for cp in iter_all_chars() {
        if BidiClass::of(cp) == BidiClass::CommonSeparator {
            assert!(
                GC::of(cp) == GC::OtherPunctuation  ||
                GC::of(cp) == GC::SpaceSeparator    ||
                GC::of(cp) == GC::MathSymbol
            );
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
        if BidiClass::of(cp) == BidiClass::NonspacingMark {
            assert!(is_combining_mark(cp));
        }
    }

    // Every GC!=Mark must not be an NSM
    for cp in iter_all_chars() {
        if !is_combining_mark(cp) {
            assert_ne!(BidiClass::of(cp), BidiClass::NonspacingMark);
        }
    }
}

/// `Bidi_Class=BN := General_Category in { Cc (Control), Co (PrivateUse) }`
///
/// <http://www.unicode.org/reports/tr9/#BN>
#[test]
fn test_bidi_bn_against_gen_cat() {
    for cp in iter_all_chars() {
        // Every BoundaryNeutral must satisfy GC::is_other()
        if BidiClass::of(cp) == BidiClass::BoundaryNeutral {
            assert!(GC::of(cp).is_other());
        }

        // Every !GC::is_other() must not be an BoundaryNeutral
        if !GC::of(cp).is_other() {
            assert_ne!(BidiClass::of(cp), BidiClass::BoundaryNeutral);
        }
    }
}

/// `Bidi_Class=B := General_Category in { Cc (Control), Zp (ParagraphSeparator) }`
///
/// <http://www.unicode.org/reports/tr9/#B>
#[test]
fn test_bidi_b_against_gen_cat() {
    // Every ParagraphSeparator must be a GC::Control or GC::ParagraphSeparator
    for cp in iter_all_chars() {
        if BidiClass::of(cp) == BidiClass::ParagraphSeparator {
            assert!(
                GC::of(cp) == GC::Control ||
                GC::of(cp) == GC::ParagraphSeparator
            );
        }
    }
}

/// `Bidi_Class=S := General_Category in { Cc (Control) }`
///
/// <http://www.unicode.org/reports/tr9/#S>
#[test]
fn test_bidi_s_against_gen_cat() {
    // Every Segment Separator must be a GC::Control
    for cp in iter_all_chars() {
        if BidiClass::of(cp) == BidiClass::SegmentSeparator {
            assert!(GC::of(cp) == GC::Control);
        }
    }
}

/// `Bidi_Class=WS := General_Category in { Cc (Control), Zs (SpaceSeparator), Zl (LineSeparator) }`
///
/// <http://www.unicode.org/reports/tr9/#WS>
#[test]
fn test_bidi_ws_against_gen_cat() {
    // Every BC::WhiteSpace must be a GC::Control, GC::SpaceSeparator, or GC::LineSeparator
    for cp in iter_all_chars() {
        if BidiClass::of(cp) == BidiClass::WhiteSpace {
            assert!(
                GC::of(cp) == GC::Control           ||
                GC::of(cp) == GC::SpaceSeparator    ||
                GC::of(cp) == GC::LineSeparator
            );
        }
    }
}
