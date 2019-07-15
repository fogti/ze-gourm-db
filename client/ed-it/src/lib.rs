#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate failure;
extern crate num;
extern crate num_traits as _num_traits;
#[macro_use]
extern crate num_derive;
extern crate phf;

use std::collections::HashMap;
use std::fmt;
use std::io::{BufRead, BufReader, Read};
use phf::phf_map;

#[derive(Debug)]
pub enum RecipeCategory {
    Custom(usize, String),
}

#[derive(Debug)]
pub struct Recipe {
    name: String,
    category: RecipeCategory,
    groceries: Vec<String>, // for now, change that later
    making: Vec<String>,
    notes: Vec<String>,
    sources: Vec<String>,
}

impl fmt::Display for RecipeCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecipeCategory::Custom(_, x) => write!(f, "{}", x),
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, FromPrimitive)]
pub enum RecipeLineType {
    // All misc lines
    Normal = 0,
    // single-line-values (K:V)
    Name = 1,
    Category = 2,
    // multi-line-section-headers (%K)
    Groceries = 3,
    Making = 4,
    Notes = 5,
    Sources = 6,
}

/// Yes, this is hacky, but we can't use phf_map because of some
/// limitations of phf_derive
/// (e.g.: "unsupported key expression")
///
/// WE NEED TO MANUALLY UPDATE THIS TO MATCH RecipeLineType from above!
static DEFAULT_RSM_BOUNDS: std::ops::Range<i32> = 1..7;

/// Important NOTE: this table must be manually synchonized to the
/// type definition above.
///
/// This approach is probably relatively good,
/// we get checks for 'non-exhaustive patterns',
/// so we can't miss an update here
fn get_default_rsm(x: RecipeLineType) -> &'static str {
    macro_rules! helper {
        ($($elem:ident),+) => {
            match x {
                RecipeLineType::Normal => "",
                $(RecipeLineType::$elem => stringify!($elem),)+
            }
        }
    }
    helper!(Name, Category, Groceries, Making, Notes, Sources)
}

/// mappings : LocalizedString -> RecipeLineType
static LOC_MAPPINGS: phf::Map<&'static str, RecipeLineType> = phf_map! {
    "name" => RecipeLineType::Name,
    "category" => RecipeLineType::Category,
    "kategorie" => RecipeLineType::Category,
    "groceries" => RecipeLineType::Groceries,
    "zutaten" => RecipeLineType::Groceries,
    "making" => RecipeLineType::Making,
    "zubereitung" => RecipeLineType::Making,
    "bemerkungen" => RecipeLineType::Notes,
    "notes" => RecipeLineType::Notes,
    "notizen" => RecipeLineType::Notes,
    "sources" => RecipeLineType::Sources,
    "quellen" => RecipeLineType::Sources,
};

#[derive(Debug, Fail)]
enum RecipeError {
    #[fail(display = "got line with invalid single (K:V) value: {}", _0)]
    InvalidSingleValue(String),
    #[fail(display = "got line with invalid multiline header: {}", _0)]
    InvalidMultilineValue(String),
}

pub fn parse_recipe_from_file<FT: Read>(r: FT) -> Result<Recipe, failure::Error> {
    let mappings = &LOC_MAPPINGS;

    let mut ret = Recipe {
        name: String::new(),
        category: RecipeCategory::Custom(0, "null".to_string()),
        groceries: vec![],
        making: vec![],
        notes: vec![],
        sources: vec![],
    };
    let mut mode = RecipeLineType::Normal;
    for i in BufReader::new(r).lines() {
        let mut i = i?;
        // filter comments
        if let Some(cmt_start) = i.find('#') {
            i = i.drain(..cmt_start).collect();
        }
        let i = i.trim_end();
        // filter lines without content
        if i.is_empty() {
            continue;
        }

        if i.chars().nth(0) == Some('%') {
            // parse multiline section-start
            let imlve = || RecipeError::InvalidMultilineValue(i.to_string()).into();
            if i.len() == 1 {
                return Err(imlve());
            }
            let tmp = i[1..].to_string().to_lowercase();
            let rlt = mappings.get(&*tmp).ok_or_else(imlve)?;
            match rlt {
                RecipeLineType::Groceries
                | RecipeLineType::Making
                | RecipeLineType::Notes
                | RecipeLineType::Sources => {}
                _ => return Err(imlve()),
            }
            mode = *rlt;
            continue;
        }
        let mut i = i.to_string();
        match mode {
            RecipeLineType::Normal => {
                // parse single K:V line
                let isve = {
                    let i = i.clone();
                    move || RecipeError::InvalidSingleValue(i).into()
                };
                if i.len() == 1 {
                    return Err(isve());
                }

                match i.find(':') {
                    None => return Err(isve()),
                    Some(dcpos) => {
                        let key = i.drain(..dcpos).collect::<String>().to_lowercase();
                        if i.is_empty() {
                            return Err(isve());
                        }
                        let value = i[1..].trim_start().to_string();
                        mappings
                            .get(&*key)
                            .and_then(|rlt| {
                                match rlt {
                                    RecipeLineType::Name => ret.name = value,
                                    RecipeLineType::Category => {
                                        ret.category = RecipeCategory::Custom(0, value)
                                    } // TODO
                                    _ => return None,
                                }
                                Some(())
                            })
                            .ok_or_else(isve)?;
                    }
                }
            }
            RecipeLineType::Groceries => ret.groceries.push(i),
            RecipeLineType::Making => ret.making.push(i),
            RecipeLineType::Notes => ret.notes.push(i),
            RecipeLineType::Sources => ret.sources.push(i),
            _ => return Err(RecipeError::InvalidSingleValue(i.to_string()).into()),
        }
    }
    Ok(ret)
}

type RSM<'a> = HashMap<RecipeLineType, &'a str>;

impl Recipe {
    fn rts_single(ret: &mut String, mappings: &RSM, part: &str, rlt: RecipeLineType) {
        if !part.is_empty() {
            *ret += mappings.get(&rlt).unwrap();
            *ret += ": ";
            *ret += part;
            *ret += "\n";
        }
    }

    fn rts_multiline(ret: &mut String, mappings: &RSM, part: &[String], rlt: RecipeLineType) {
        if !part.is_empty() {
            let sechead = mappings.get(&rlt).unwrap();
            ret.reserve(sechead.len() + 3 + part.len() * 8);
            *ret += "\n%";
            *ret += sechead;
            *ret += "\n";
            for i in part {
                *ret += i;
                *ret += "\n";
            }
        }
    }

    pub fn to_string(&self, mut mappings: RSM) -> String {
        let mut ret = String::new();
        // make sure all required RLT are handled
        for i in DEFAULT_RSM_BOUNDS.clone() {
            let rlt: RecipeLineType = num::FromPrimitive::from_i32(i).unwrap();
            mappings.entry(rlt).or_insert(get_default_rsm(rlt));
        }
        macro_rules! rtshelper {
            ($fn:ident, $part:tt, $rlt:ident) => {
                Self::$fn(&mut ret, &mappings, &self.$part, RecipeLineType::$rlt)
            }
        }
        rtshelper!(rts_single, name, Name);
        Self::rts_single(&mut ret, &mappings, &self.category.to_string(), RecipeLineType::Category);
        rtshelper!(rts_multiline, groceries, Groceries);
        rtshelper!(rts_multiline, making, Making);
        rtshelper!(rts_multiline, notes, Notes);
        rtshelper!(rts_multiline, sources, Sources);
        ret
    }
}
