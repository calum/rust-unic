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
/// `normal::is_combining_mark` and `GeneralCategory::is_mark()` are expected to return
/// the same results.
#[test]
fn test_gen_cat_against_normal() {
    // Every General_Category mark must be a combining mark
    for cp in iter_all_chars() {
        if GC::of(cp).is_mark() {
            assert!(is_combining_mark(cp));
        }
    }

    // Every combining mark must be a General_Category mark
    for cp in iter_all_chars() {
        if is_combining_mark(cp) {
            assert!(GC::of(cp).is_mark());
        }
    }
}
