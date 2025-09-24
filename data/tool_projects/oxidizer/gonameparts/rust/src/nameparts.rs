use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct NameParts {
    #[serde(rename = "ProvidedName")]
    pub provided_name: String,
    #[serde(rename = "FullName")]
    pub full_name: String,
    #[serde(rename = "Salutation")]
    pub salutation: String,
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "MiddleName")]
    pub middle_name: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
    #[serde(rename = "Generation")]
    pub generation: String,
    #[serde(rename = "Suffix")]
    pub suffix: String,
    #[serde(rename = "Nickname")]
    pub nickname: String,
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "Aliases")]
    pub aliases: Vec<NameParts>,
}

#[cfg(not(feature = "mock"))]
impl NameParts {
    pub(crate) fn build_full_name(&mut self) -> Result<(), Error> {
        let mut full_name_parts = Vec::new();
        if !self.salutation.is_empty() {
            full_name_parts.push(self.salutation.clone());
        }
        if !self.first_name.is_empty() {
            full_name_parts.push(self.first_name.clone());
        }
        if !self.middle_name.is_empty() {
            full_name_parts.push(self.middle_name.clone());
        }
        if !self.last_name.is_empty() {
            full_name_parts.push(self.last_name.clone());
        }
        if !self.generation.is_empty() {
            full_name_parts.push(self.generation.clone());
        }
        if !self.suffix.is_empty() {
            full_name_parts.push(self.suffix.clone());
        }
        self.full_name = full_name_parts.join(" ");
        Ok(())
    }
}
#[cfg(feature = "mock")]
impl NameParts {
    pub(crate) fn build_full_name(&mut self) -> Result<(), Error> {
        extern "C" {
            #[link_name = "gonameparts_name_parts___build_full_name__ground_truth"]
            fn NameParts_build_full_name__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut NameParts);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameParts>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NameParts_build_full_name__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
            };
            let input_state_mutated: InputStateOut = serde_json::from_value(
                    inputs_mutation_reserialized,
                )
                .unwrap();
            *self = *input_state_mutated.0;
            let output_state: OutputState = serde_json::from_value(
                    foreign_execution.return_value,
                )
                .unwrap();
            let output = ();
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl NameParts {
    pub(crate) fn build_full_name__with_callees_mocked(&mut self) -> Result<(), Error> {
        let mut full_name_parts = Vec::new();
        if !self.salutation.is_empty() {
            full_name_parts.push(self.salutation.clone());
        }
        if !self.first_name.is_empty() {
            full_name_parts.push(self.first_name.clone());
        }
        if !self.middle_name.is_empty() {
            full_name_parts.push(self.middle_name.clone());
        }
        if !self.last_name.is_empty() {
            full_name_parts.push(self.last_name.clone());
        }
        if !self.generation.is_empty() {
            full_name_parts.push(self.generation.clone());
        }
        if !self.suffix.is_empty() {
            full_name_parts.push(self.suffix.clone());
        }
        self.full_name = full_name_parts.join(" ");
        Ok(())
    }
}

use std::string::ToString;
#[cfg(not(feature = "mock"))]
impl NameParts {
    pub(crate) fn slot(&mut self, part: &str, value: &str) {
        match part {
            "salutation" => self.salutation = value.to_string(),
            "generation" => self.generation = value.to_string(),
            "suffix" => self.suffix = value.to_string(),
            "middle" => self.middle_name = value.to_string(),
            "last" => self.last_name = value.to_string(),
            "first" => self.first_name = value.to_string(),
            _ => {}
        }
    }
}
#[cfg(feature = "mock")]
impl NameParts {
    pub(crate) fn slot(&mut self, part: &str, value: &str) {
        extern "C" {
            #[link_name = "gonameparts_name_parts___slot__ground_truth"]
            fn NameParts_slot__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b, 'c>(&'a mut NameParts, &'b str, &'c str);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameParts>, Box<str>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self, part, value);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NameParts_slot__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 3usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
            };
            let input_state_mutated: InputStateOut = serde_json::from_value(
                    inputs_mutation_reserialized,
                )
                .unwrap();
            *self = *input_state_mutated.0;
            let output_state: OutputState = serde_json::from_value(
                    foreign_execution.return_value,
                )
                .unwrap();
            let output = ();
            return output;
        } else {
            panic!("execution failure");
        }
    }
}
#[cfg(feature = "mock")]
impl NameParts {
    pub(crate) fn slot__with_callees_mocked(&mut self, part: &str, value: &str) {
        match part {
            "salutation" => self.salutation = value.to_string(),
            "generation" => self.generation = value.to_string(),
            "suffix" => self.suffix = value.to_string(),
            "middle" => self.middle_name = value.to_string(),
            "last" => self.last_name = value.to_string(),
            "first" => self.first_name = value.to_string(),
            _ => {}
        }
    }
}

use once_cell::sync::Lazy;
pub(crate) static GENERATIONS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "JR", "SR", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "1ST",
        "2ND", "3RD", "4TH", "5TH", "6TH", "7TH", "8TH", "9TH", "10TH", "FIRST",
        "SECOND", "THIRD", "FOURTH", "FIFTH", "SIXTH", "SEVENTH", "EIGHTH", "NINTH",
        "TENTH",
    ]
});

pub(crate) static LN_PREFIXES: Lazy<Vec<String>> = Lazy::new(|| {
    vec![
        "DE".to_string(), "DA".to_string(), "DI".to_string(), "LA".to_string(), "DU"
        .to_string(), "DEL".to_string(), "DEI".to_string(), "VDA".to_string(), "DELLO"
        .to_string(), "DELLA".to_string(), "DEGLI".to_string(), "DELLE".to_string(),
        "VAN".to_string(), "VON".to_string(), "DER".to_string(), "DEN".to_string(),
        "HEER".to_string(), "TEN".to_string(), "TER".to_string(), "VANDE".to_string(),
        "VANDEN".to_string(), "VANDER".to_string(), "VOOR".to_string(), "VER"
        .to_string(), "AAN".to_string(), "MC".to_string(), "BEN".to_string(), "SAN"
        .to_string(), "SAINZ".to_string(), "BIN".to_string(), "LI".to_string(), "LE"
        .to_string(), "DES".to_string(), "AM".to_string(), "AUS'M".to_string(), "VOM"
        .to_string(), "ZUM".to_string(), "ZUR".to_string(), "TEN".to_string(), "IBN"
        .to_string(),
    ]
});

pub(crate) static NON_NAME: &'static [&'static str] = &[
    "A.K.A",
    "AKA",
    "A/K/A",
    "F.K.A",
    "FKA",
    "F/K/A",
    "N/K/A",
];

pub(crate) static SALUTATIONS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "MR", "MS", "MRS", "DR", "MISS", "DOCTOR", "CORP", "SGT", "PVT", "JUDGE", "CAPT",
        "COL", "MAJ", "LT", "LIEUTENANT", "PRM", "PATROLMAN", "HON", "OFFICER", "REV",
        "PRES", "PRESIDENT", "GOV", "GOVERNOR", "VICE PRESIDENT", "VP", "MAYOR", "SIR",
        "MADAM", "HONORABLE",
    ]
});

pub(crate) static SUFFIXES: &[&str] = &["ESQ", "PHD", "MD"];

pub(crate) static SUPPLEMENTAL_INFO: [&'static str; 6] = [
    "WIFE OF",
    "HUSBAND OF",
    "SON OF",
    "DAUGHTER OF",
    "DECEASED",
    "FICTITIOUS",
];

pub(crate) static CORP_ENTITY: [&str; 17] = [
    "NA",
    "CORP",
    "CO",
    "INC",
    "ASSOCIATES",
    "SERVICE",
    "LLC",
    "LLP",
    "PARTNERS",
    "R/A",
    "C/O",
    "COUNTY",
    "STATE",
    "BANK",
    "GROUP",
    "MUTUAL",
    "FARGO",
];
use crate::namestring::NameString;
use std::collections::HashMap;
#[cfg(not(feature = "mock"))]
pub fn parse(name: &str) -> Result<NameParts, anyhow::Error> {
    let mut n = NameString {
        full_name: name.to_string(),
        ..Default::default()
    };
    n.normalize()?;
    let mut p = NameParts {
        provided_name: name.to_string(),
        nickname: n.nickname.clone(),
        ..Default::default()
    };
    if n.looks_corporate() {
        return Ok(p);
    }
    let parts = vec!["generation", "suffix", "lnprefix", "supplemental"];
    let mut part_map: HashMap<&str, i32> = HashMap::new();
    let mut slotted: Vec<usize> = Vec::new();
    for part in &parts {
        let part_index = n.find(part)?;
        part_map.insert(part, part_index);
        if part_index > -1 {
            p.slot(part, &n.split_name[part_index as usize]);
            slotted.push(part_index as usize);
        }
    }
    if let Ok(sal_index) = n.find("salutation") {
        if sal_index == 0 {
            part_map.insert("salutation", sal_index);
            p.slot("salutation", &n.split_name[sal_index as usize]);
            slotted.push(sal_index as usize);
        } else {
            part_map.insert("salutation", -1);
        }
    } else {
        part_map.insert("salutation", -1);
    }
    if let Ok(nn_index) = n.find("nonname") {
        if nn_index > -1 && nn_index < (n.split_name.len() - 1) as i32 {
            part_map.insert("nonname", nn_index);
            p.slot("nonname", &n.split_name[nn_index as usize]);
            slotted.push(nn_index as usize);
        } else {
            part_map.insert("nonname", -1);
        }
    } else {
        part_map.insert("nonname", -1);
    }
    let first_pos = *part_map.get("salutation").unwrap_or(&-1) + 1;
    if first_pos == n.split_name.len() as i32 {
        p.build_full_name()?;
        return Ok(p);
    }
    part_map.insert("first", first_pos);
    p.slot("first", &n.split_name[first_pos as usize]);
    slotted.push(first_pos as usize);
    if let Some(&ln_prefix) = part_map.get("lnprefix") {
        if ln_prefix > -1 {
            let mut ln_end = n.split_name.len();
            if let Some(&generation) = part_map.get("generation") {
                if generation > -1 {
                    ln_end = generation as usize;
                }
            }
            if let Some(&suffix) = part_map.get("suffix") {
                if suffix > -1 && (suffix as usize) < ln_end {
                    ln_end = suffix as usize;
                }
            }
            if ln_end > ln_prefix as usize {
                let last_name: Vec<&str> = n
                    .split_name[ln_prefix as usize..ln_end]
                    .iter()
                    .map(|s| s.as_str())
                    .collect();
                p.slot("last", &last_name.join(" "));
            }
            for i in ln_prefix as usize..=ln_end {
                slotted.push(i);
            }
        }
    }
    let not_slotted = n.find_not_slotted(&slotted)?;
    if not_slotted.len() > 1 {
        let ln_prefix = *part_map.get("lnprefix").unwrap_or(&-1);
        let mut multi_middle = Vec::new();
        if ln_prefix > -1 {
            for p in &not_slotted {
                multi_middle.push(n.split_name[*p].clone());
            }
            p.slot("middle", &multi_middle.join(" "));
        } else {
            let mut not_slotted_sorted = not_slotted.clone();
            not_slotted_sorted.sort_unstable();
            let max_not_slotted_index = *not_slotted_sorted.last().unwrap();
            p.slot("last", &n.split_name[max_not_slotted_index]);
            for p in &not_slotted {
                if *p != max_not_slotted_index {
                    multi_middle.push(n.split_name[*p].clone());
                }
            }
            p.slot("middle", &multi_middle.join(" "));
        }
    }
    if not_slotted.len() == 1 {
        if *part_map.get("lnprefix").unwrap_or(&-1) > -1 {
            p.slot("middle", &n.split_name[not_slotted[0]]);
        } else {
            p.slot("last", &n.split_name[not_slotted[0]]);
        }
    }
    for alias in &n.aliases {
        p.aliases.push(parse(alias)?);
    }
    p.build_full_name()?;
    Ok(p)
}
#[cfg(feature = "mock")]
pub fn parse(name: &str) -> Result<NameParts, anyhow::Error> {
    extern "C" {
        #[link_name = "gonameparts_parse__ground_truth"]
        fn parse__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a str);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(NameParts);
    let input_state_in = InputStateIn(name);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(parse__foreign(ser(&input_state_in)))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 1usize);
        let inputs_mutation_reserialized = if foreign_execution.input_modifications.len()
            == 1
        {
            foreign_execution.input_modifications[0].clone()
        } else {
            serde_json::to_value(foreign_execution.input_modifications.clone()).unwrap()
        };
        let input_state_mutated: InputStateOut = serde_json::from_value(
                inputs_mutation_reserialized,
            )
            .unwrap();
        let output_state: OutputState = serde_json::from_value(
                foreign_execution.return_value,
            )
            .unwrap();
        let output = output_state.0;
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn parse__with_callees_mocked(name: &str) -> Result<NameParts, anyhow::Error> {
    let mut n = NameString {
        full_name: name.to_string(),
        ..Default::default()
    };
    n.normalize()?;
    let mut p = NameParts {
        provided_name: name.to_string(),
        nickname: n.nickname.clone(),
        ..Default::default()
    };
    if n.looks_corporate() {
        return Ok(p);
    }
    let parts = vec!["generation", "suffix", "lnprefix", "supplemental"];
    let mut part_map: HashMap<&str, i32> = HashMap::new();
    let mut slotted: Vec<usize> = Vec::new();
    for part in &parts {
        let part_index = n.find(part)?;
        part_map.insert(part, part_index);
        if part_index > -1 {
            p.slot(part, &n.split_name[part_index as usize]);
            slotted.push(part_index as usize);
        }
    }
    if let Ok(sal_index) = n.find("salutation") {
        if sal_index == 0 {
            part_map.insert("salutation", sal_index);
            p.slot("salutation", &n.split_name[sal_index as usize]);
            slotted.push(sal_index as usize);
        } else {
            part_map.insert("salutation", -1);
        }
    } else {
        part_map.insert("salutation", -1);
    }
    if let Ok(nn_index) = n.find("nonname") {
        if nn_index > -1 && nn_index < (n.split_name.len() - 1) as i32 {
            part_map.insert("nonname", nn_index);
            p.slot("nonname", &n.split_name[nn_index as usize]);
            slotted.push(nn_index as usize);
        } else {
            part_map.insert("nonname", -1);
        }
    } else {
        part_map.insert("nonname", -1);
    }
    let first_pos = *part_map.get("salutation").unwrap_or(&-1) + 1;
    if first_pos == n.split_name.len() as i32 {
        p.build_full_name()?;
        return Ok(p);
    }
    part_map.insert("first", first_pos);
    p.slot("first", &n.split_name[first_pos as usize]);
    slotted.push(first_pos as usize);
    if let Some(&ln_prefix) = part_map.get("lnprefix") {
        if ln_prefix > -1 {
            let mut ln_end = n.split_name.len();
            if let Some(&generation) = part_map.get("generation") {
                if generation > -1 {
                    ln_end = generation as usize;
                }
            }
            if let Some(&suffix) = part_map.get("suffix") {
                if suffix > -1 && (suffix as usize) < ln_end {
                    ln_end = suffix as usize;
                }
            }
            if ln_end > ln_prefix as usize {
                let last_name: Vec<&str> = n
                    .split_name[ln_prefix as usize..ln_end]
                    .iter()
                    .map(|s| s.as_str())
                    .collect();
                p.slot("last", &last_name.join(" "));
            }
            for i in ln_prefix as usize..=ln_end {
                slotted.push(i);
            }
        }
    }
    let not_slotted = n.find_not_slotted(&slotted)?;
    if not_slotted.len() > 1 {
        let ln_prefix = *part_map.get("lnprefix").unwrap_or(&-1);
        let mut multi_middle = Vec::new();
        if ln_prefix > -1 {
            for p in &not_slotted {
                multi_middle.push(n.split_name[*p].clone());
            }
            p.slot("middle", &multi_middle.join(" "));
        } else {
            let mut not_slotted_sorted = not_slotted.clone();
            not_slotted_sorted.sort_unstable();
            let max_not_slotted_index = *not_slotted_sorted.last().unwrap();
            p.slot("last", &n.split_name[max_not_slotted_index]);
            for p in &not_slotted {
                if *p != max_not_slotted_index {
                    multi_middle.push(n.split_name[*p].clone());
                }
            }
            p.slot("middle", &multi_middle.join(" "));
        }
    }
    if not_slotted.len() == 1 {
        if *part_map.get("lnprefix").unwrap_or(&-1) > -1 {
            p.slot("middle", &n.split_name[not_slotted[0]]);
        } else {
            p.slot("last", &n.split_name[not_slotted[0]]);
        }
    }
    for alias in &n.aliases {
        p.aliases.push(parse(alias)?);
    }
    p.build_full_name()?;
    Ok(p)
}

