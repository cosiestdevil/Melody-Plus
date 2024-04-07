use serde::{Deserialize, Serialize};

/// The precision with which `release_date` value is known.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReleaseDatePrecision {
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "day")]
    Day,
}

impl Default for ReleaseDatePrecision {
    fn default() -> ReleaseDatePrecision {
        Self::Year
    }
}