#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate failure;
extern crate phf;
extern crate spawn_editor;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use phf::phf_map;

#[derive(Debug)]
enum RecipeCategory {
    Custom(usize, String),
}

#[derive(Debug)]
struct Recipe {
    name: String,
    category: RecipeCategory,
    groceries: Vec<String>, // for now, change that later
    making: Vec<String>,
    notes: Vec<String>,
    sources: Vec<String>,
}

#[derive(Copy, Clone, PartialEq)]
enum RecipeLineType {
    // All misc lines
    Normal,
    // single-line-values (K:V)
    Name,
    Category,
    // multi-line-section-headers (%K)
    Groceries,
    Making,
    Notes,
    Sources,
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

fn parse_recipe_from_file<FT: Read>(r: FT) -> Result<Recipe, failure::Error> {
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
                RecipeLineType::Groceries | RecipeLineType::Making | RecipeLineType::Notes | RecipeLineType::Sources => {}
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
                        mappings.get(&*key).and_then(|rlt| {
                            match rlt {
                                RecipeLineType::Name => ret.name = value,
                                RecipeLineType::Category => ret.category = RecipeCategory::Custom(0, value), // TODO
                                _ => return None,
                            }
                            Some(())
                        }).ok_or_else(isve)?;
                    },
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

fn main() {
    let f = File::open("testfile.txt").expect("file-open failed");
    println!("{:#?}", parse_recipe_from_file(f).expect("parsing failed"));
}
