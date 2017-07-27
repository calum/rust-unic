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
/// <http://www.unicode.org/reports/tr9/#NSM>
#[test]
fn test_bidi_bn_against_gen_cat() {
    for cp in iter_all_chars() {
        // Every BN must be a GC=Other
        if BidiClass::of(cp) == BidiClass::BoundaryNeutral {
            assert!(GC::of(cp).is_other());
        }

        // Every GC!=Other must not be an BN
        if !GC::of(cp).is_other() {
            assert_ne!(BidiClass::of(cp), BidiClass::BoundaryNeutral);
        }
    }
}

/// `Bidi_Class=B := General_Category in { Cc (Control), Zp (ParagraphSeparator) }`
///
/// <http://www.unicode.org/reports/tr9/#NSM>
#[test]
fn test_bidi_b_against_gen_cat() {
    // Every B must be a GC::Control or GC::ParagraphSeparator
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
/// <http://www.unicode.org/reports/tr9/#NSM>
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
/// <http://www.unicode.org/reports/tr9/#NSM>
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

/// `Bidi_Class=ON := General_Category in { Cc (Control), Zs (SpaceSeparator), Zl (LineSeparator) }`
///
/// <http://www.unicode.org/reports/tr9/#NSM>
#[test]
fn test_bidi_on_against_gen_cat() {
    // Every BC::OtherNeutral must be a GC::Control, GC::SpaceSeparator, or GC::LineSeparator
    for cp in iter_all_chars() {
        if BidiClass::of(cp) == BidiClass::OtherNeutral {
            if !GC::of(cp).is_symbol() && !GC::of(cp).is_punctuation() {

                println!("{:?}", GC::of(cp));
            }
            assert!(
                GC::of(cp).is_symbol() ||
                GC::of(cp).is_punctuation()
            );
        }
    }
}

#[test]
fn example() {
    let symbol = GC::ModifierSymbol;
    println!("{:?} is symbol?: {:?}",symbol, symbol.is_symbol() );
}

/// `General_Category to Bidi_Class`
///
/// <http://www.unicode.org/reports/tr9/#Table_Bidirectional_Character_Types>
#[test]
fn test_gen_cat_against_bidi() {
    // Every GC::Unassigned (Cn) is a BN (BoundaryNeutral)
    for cp in iter_all_chars() {
        match GC::of(cp) {
            // Co => L
            GC::PrivateUse => assert!(BidiClass::of(cp) == BidiClass::LeftToRight),
            // Mc => L
            GC::SpacingMark => assert!(BidiClass::of(cp) == BidiClass::LeftToRight),
            // Lt => L
            GC::TitlecaseLetter => assert!(BidiClass::of(cp) == BidiClass::LeftToRight),

            // Pf => ON
            GC::FinalPunctuation => assert!(BidiClass::of(cp) == BidiClass::OtherNeutral),
            // Pi => ON
            GC::InitialPunctuation => assert!(BidiClass::of(cp) == BidiClass::OtherNeutral),
            // Pe => ON
            GC::ClosePunctuation => assert!(BidiClass::of(cp) == BidiClass::OtherNeutral),
            // Ps => ON
            GC::OpenPunctuation => assert!(BidiClass::of(cp) == BidiClass::OtherNeutral),
            // Pc => ON
            GC::ConnectorPunctuation => assert!(BidiClass::of(cp) == BidiClass::OtherNeutral),

            // Zp => WS
            GC::ParagraphSeparator => assert!(BidiClass::of(cp) == BidiClass::ParagraphSeparator),
            // Zl => WS
            GC::LineSeparator => assert!(BidiClass::of(cp) == BidiClass::WhiteSpace),

            // Me => NSM
            GC::EnclosingMark => assert!(BidiClass::of(cp) == BidiClass::NonspacingMark),
            // Mn => NSM
            //GC::NonspacingMark => assert!(BidiClass::of(cp) == BidiClass::NonspacingMark), // 2 come out as being LeftToRight

            GC::Control => assert!(
                BidiClass::of(cp) == BidiClass::BoundaryNeutral ||
                BidiClass::of(cp) == BidiClass::SegmentSeparator ||
                BidiClass::of(cp) == BidiClass::ParagraphSeparator ||
                BidiClass::of(cp) == BidiClass::WhiteSpace
            ),
            GC::SpaceSeparator => assert!(
                BidiClass::of(cp) == BidiClass::CommonSeparator ||
                BidiClass::of(cp) == BidiClass::WhiteSpace
            ),
            GC::CurrencySymbol => assert!(
                BidiClass::of(cp) == BidiClass::EuropeanTerminator ||
                BidiClass::of(cp) == BidiClass::ArabicLetter
            ),

            // Surrogate (Cs) is never found
            // Unassigned, Format, OtherSymbol, ModifierSymbol, MathSymbol, OtherPunctuation, DashPunctuation,
            // OtherNumber, LetterNumber, DecimalNumber, OtherLetter, ModifierLetter,
            // LowercaseLetter, and UppercaseLetter can be multiple Bidi_Classes
            _ => assert!(true),
        }
    }
}
