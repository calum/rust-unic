use std::char;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::{UnicodeData, UnicodeDataEntry, UnicodeVersion};

use generate::PREAMBLE;
use generate::char_property::ToRangeBSearchMap;

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CategoryData<'a>(BTreeMap<char, &'a str>);

impl<'a> CategoryData<'a> {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let CategoryData(ref map) = *self;
        let mut file = File::create(dir.as_ref().join("general_category.rsv"))?;
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            map.to_range_bsearch_map(Display::fmt)
        )?;
        Ok(())
    }
}

impl<'a, I> From<I> for CategoryData<'a>
where
    I: Iterator<Item = &'a UnicodeDataEntry>,
{
    fn from(it: I) -> Self {
        let mut map = BTreeMap::<char, &str>::new();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        for &UnicodeDataEntry { character, ref general_category, .. } in it {
            map.insert(character, general_category);
        }

        CategoryData(map)
    }
}

/// Generate tables for the ucd-category crate
pub fn generate<P: AsRef<Path>>(
    dir: P,
    version: &UnicodeVersion,
    data: &UnicodeData,
) -> io::Result<()> {
    println!("> unic::ucd::category::tables::unicode_version");
    version.emit(&dir)?;
    println!("> unic::ucd::category::tables::general_category");
    CategoryData::from(data.iter()).emit(dir)?;
    Ok(())
}
