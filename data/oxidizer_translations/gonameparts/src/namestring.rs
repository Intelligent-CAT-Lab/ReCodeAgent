#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/polera/gonameparts.nameString
#[derive(Default)]#[derive(Clone)]pub(crate) struct NameString {
    pub full_name: String,
    pub split_name: Vec<String>,
    pub nickname: String,
    pub aliases: Vec<String>,
}

use std::str;
//Translated from: (*github.com/polera/gonameparts.nameString).split
impl NameString {
    pub(crate) fn split(&mut self) -> &Vec<String> {
        self.split_name = str::split_whitespace(&self.full_name)
            .map(|s| s.to_string())
            .collect();
        &self.split_name
    }
}

//Translated from: (*github.com/polera/gonameparts.nameString).cleaned
impl NameString {
    pub(crate) fn cleaned(&mut self) -> Vec<String> {
        let unwanted = vec![",", "."];
        let mut cleaned = Vec::new();

        for x in self.split() {
            let mut x = x.clone();
            for y in &unwanted {
                x = x.replace(y, "");
            }
            cleaned.push(x.trim().to_string());
        }

        cleaned
    }
}
use std::cmp::Ordering;
//Translated from: (*github.com/polera/gonameparts.nameString).searchParts
impl NameString {
    pub(crate) fn search_parts(&mut self, parts: &[String]) -> i32 {
        for (i, x) in self.cleaned().iter().enumerate() {
            for y in parts {
                if x.to_uppercase() == y.to_uppercase() {
                    return i as i32;
                }
            }
        }
        -1
    }
}
use crate::nameparts::GENERATIONS;
use crate::nameparts::LN_PREFIXES;
use crate::nameparts::NON_NAME;
use crate::nameparts::SALUTATIONS;
use crate::nameparts::SUFFIXES;
use crate::nameparts::SUPPLEMENTAL_INFO;
//Translated from: (*github.com/polera/gonameparts.nameString).find
impl NameString {
    pub(crate) fn find(&mut self, part: &str) -> Result<i32, Error> {
        match part {
            "salutation" => Ok(self.search_parts(&SALUTATIONS.iter().map(|s| s.to_string()).collect::<Vec<_>>())),
            "generation" => Ok(self.search_parts(&GENERATIONS.iter().map(|s| s.to_string()).collect::<Vec<_>>())),
            "suffix" => Ok(self.search_parts(&SUFFIXES.iter().map(|s| s.to_string()).collect::<Vec<_>>())),
            "lnprefix" => Ok(self.search_parts(&LN_PREFIXES)),
            "nonname" => Ok(self.search_parts(&NON_NAME.iter().map(|s| s.to_string()).collect::<Vec<_>>())),
            "supplemental" => Ok(self.search_parts(&SUPPLEMENTAL_INFO.iter().map(|s| s.to_string()).collect::<Vec<_>>())),
            _ => Ok(-1),
        }
    }
}

//Translated from: (*github.com/polera/gonameparts.nameString).findNotSlotted
impl NameString {
    pub(crate) fn find_not_slotted(&self, slotted: &[usize]) -> Result<Vec<usize>> {
        let mut not_slotted = Vec::new();

        for (i, _) in self.split_name.iter().enumerate() {
            if !slotted.contains(&i) {
                not_slotted.push(i);
            }
        }

        Ok(not_slotted)
    }
}

//Translated from: (*github.com/polera/gonameparts.nameString).fixMisplacedApostrophe
impl NameString {
    pub(crate) fn fix_misplaced_apostrophe(&mut self) -> Result<()> {
        let mut ends_with_apostrophe = Vec::new();

        for (index, x) in self.split().iter().enumerate() {
            if x.ends_with('\'') {
                ends_with_apostrophe.push(index);
            }
        }

        if !ends_with_apostrophe.is_empty() {
            for y in ends_with_apostrophe {
                if self.split_name[y] == self.split_name[self.split_name.len() - 1] {
                    let mut tmp_name = self.split_name[..y].to_vec();
                    tmp_name.push(self.split_name[y].trim_matches('\'').to_string());
                    self.full_name = tmp_name.join(" ");
                } else {
                    let misplaced_start = y;
                    let mut fixed_name = vec![self.split_name[misplaced_start].clone()];
                    fixed_name.push(self.split_name[misplaced_start + 1].clone());
                    let fixed_placement = fixed_name.join("");

                    let mut tmp_name = self.split_name[..misplaced_start].to_vec();
                    tmp_name.push(fixed_placement);
                    tmp_name.extend_from_slice(&self.split_name[misplaced_start + 2..]);
                    self.full_name = tmp_name.join(" ");
                }
            }
        }

        Ok(())
    }
}

//Translated from: (*github.com/polera/gonameparts.nameString).hasAliases
impl NameString {
    pub(crate) fn has_aliases(&self) -> (bool, String) {
        let upper_name = self.full_name.to_uppercase();
        for x in NON_NAME.iter() {
            if upper_name.contains(x) && !upper_name.ends_with(x) {
                return (true, x.to_string());
            }
        }
        (false, String::new())
    }
}

//Translated from: (*github.com/polera/gonameparts.nameString).hasComma
impl NameString {
    pub(crate) fn has_comma(&self) -> bool {
        for part in &self.split_name {
            if str::contains(part, ",") {
                return true;
            }
        }
        false
    }
}
use crate::nameparts::CORP_ENTITY;
//Translated from: (*github.com/polera/gonameparts.nameString).looksCorporate
impl NameString {
    pub(crate) fn looks_corporate(&mut self) -> bool {
        self.search_parts(&CORP_ENTITY.iter().map(|s| s.to_string()).collect::<Vec<_>>()) >= 0
    }
}

//Translated from: (*github.com/polera/gonameparts.nameString).slotNickname
impl NameString {
    pub(crate) fn slot_nickname(&mut self) -> Result<(), Error> {
        let mut nick_name_boundaries = Vec::new();

        for (index, x) in self.split().iter().enumerate() {
            if x.starts_with('\'') || x.starts_with('\"') {
                nick_name_boundaries.push(index);
            }
            if x.ends_with('\'') || x.ends_with('\"') {
                nick_name_boundaries.push(index);
            }
        }

        if !nick_name_boundaries.is_empty() && nick_name_boundaries.len() % 2 == 0 {
            let nick_start = nick_name_boundaries[0];
            let nick_end = nick_name_boundaries[1];

            let mut nick = self.split_name[..nick_start].to_vec();
            let post_nick = self.split_name[nick_end + 1..].to_vec();

            self.nickname = self.split_name[nick_start..=nick_end].join(" ");
            nick.extend(post_nick);
            self.full_name = nick.join(" ");
        }

        Ok(())
    }
}

//Translated from: (*github.com/polera/gonameparts.nameString).splitAliases
impl NameString {
    pub(crate) fn split_aliases(&mut self, alias_sep: &str) -> Result<(), Error> {
        let split_names: Vec<_> = self.split().iter().cloned().collect();

        let transformed_names: Vec<_> = split_names
            .iter()
            .map(|part| {
                if part.to_uppercase() == alias_sep.to_uppercase() {
                    "*|*".to_string()
                } else {
                    part.clone()
                }
            })
            .collect();

        let names: Vec<_> = str::replace(&transformed_names.join(" "), "*|*", "")
            .split('*')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();

        self.full_name = names[0].clone();
        self.aliases = names[1..].to_vec();

        Ok(())
    }
}

use std::collections::HashSet;
//Translated from: (*github.com/polera/gonameparts.nameString).normalize
impl NameString {
    pub(crate) fn normalize(&mut self) -> Result<Vec<String>, Error> {
        // Handle any aliases in our nameString
        let (has_alias, alias_sep) = self.has_aliases();
        if has_alias {
            self.split_aliases(&alias_sep)?;
        }

        // Strip Supplemental info
        if let Ok(supplemental_index) = self.find("supplemental") {
            if supplemental_index > -1 {
                self.full_name = self
                    .split_name
                    .iter()
                    .take(supplemental_index as usize)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ");
            }
        }

        // Handle quoted Nicknames
        self.slot_nickname()?;

        // Handle misplaced apostrophes
        self.fix_misplaced_apostrophe()?;

        // Swap Lastname, Firstname to Firstname Lastname
        if self.has_comma() {
            let mut comma_split: Vec<&str> = self.full_name.split(',').collect();
            comma_split.swap(0, 1);
            let mut name_parts: Vec<String> = comma_split
                .into_iter()
                .map(|s| s.trim().to_string())
                .collect();
            name_parts.dedup();
            self.full_name = name_parts.join(" ");
        }

        let cleaned: Vec<String> = self.cleaned();
        Ok(cleaned)
    }
}
