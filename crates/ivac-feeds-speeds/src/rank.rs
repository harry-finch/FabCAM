//! Typed lexicographic rank; preserve the historical 12-element JSON array contract.
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "Vec<u32>", into = "[u32; 12]")]
pub struct CandidateRank {
    pub preset_tier: u32,
    pub material_cost: u32,
    pub quality_cost: u32,
    pub machine_transfer: u32,
    pub machine_specificity: u32,
    pub nonspecific_tool: u32,
    pub diameter_range: u32,
    pub nonspecific_flutes: u32,
    pub nonspecific_doc: u32,
    pub nonspecific_entry: u32,
    pub category_tier: u32,
    pub relative_delta_ppm: u32,
}
impl From<CandidateRank> for [u32; 12] {
    fn from(r: CandidateRank) -> Self {
        [
            r.preset_tier,
            r.material_cost,
            r.quality_cost,
            r.machine_transfer,
            r.machine_specificity,
            r.nonspecific_tool,
            r.diameter_range,
            r.nonspecific_flutes,
            r.nonspecific_doc,
            r.nonspecific_entry,
            r.category_tier,
            r.relative_delta_ppm,
        ]
    }
}
impl TryFrom<Vec<u32>> for CandidateRank {
    type Error = String;
    fn try_from(values: Vec<u32>) -> Result<Self, Self::Error> {
        let v: [u32; 12] = values
            .try_into()
            .map_err(|_| "Candidate rank must contain 12 costs".to_owned())?;
        Ok(Self {
            preset_tier: v[0],
            material_cost: v[1],
            quality_cost: v[2],
            machine_transfer: v[3],
            machine_specificity: v[4],
            nonspecific_tool: v[5],
            diameter_range: v[6],
            nonspecific_flutes: v[7],
            nonspecific_doc: v[8],
            nonspecific_entry: v[9],
            category_tier: v[10],
            relative_delta_ppm: v[11],
        })
    }
}
impl CandidateRank {
    pub(crate) fn same_context(&self, other: &Self) -> bool {
        let mut a = *self;
        let mut b = *other;
        a.quality_cost = 0;
        b.quality_cost = 0;
        a.relative_delta_ppm = 0;
        b.relative_delta_ppm = 0;
        a == b
    }
}
