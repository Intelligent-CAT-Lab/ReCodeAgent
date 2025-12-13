#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::data::Float64Data;
//Translated from: github.com/montanaflynn/stats.Outliers
#[derive(Default)]#[derive(Clone)]pub struct Outliers {
    pub mild: Float64Data,
    pub extreme: Float64Data,
}

use crate::quartile::Quartiles;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::quartile::inter_quartile_range;
use crate::quartile::quartile;
use crate::util::sorted_copy;
//Translated from: github.com/montanaflynn/stats.QuartileOutliers
pub fn quartile_outliers(input: Float64Data) -> Result<Outliers, anyhow::Error> {
    if input.len() == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }

    let copy = sorted_copy(input);
    let qs = quartile(copy.clone())?;
    let iqr = inter_quartile_range(copy.clone())?;

    let lif = qs.q1 - 1.5 * iqr;
    let uif = qs.q3 + 1.5 * iqr;
    let lof = qs.q1 - 3.0 * iqr;
    let uof = qs.q3 + 3.0 * iqr;

    let mut mild = Float64Data::default();
    let mut extreme = Float64Data::default();

    for v in copy.0 {
        if v < lof || v > uof {
            extreme.0.push(v);
        } else if v < lif || v > uif {
            mild.0.push(v);
        }
    }

    Ok(Outliers { mild, extreme })
}
