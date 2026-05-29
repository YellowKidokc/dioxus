use crate::types::Series;

pub fn accent(series: Series) -> &'static str {
    match series {
        Series::GTQ => "#14b8a6",
        Series::MDA => "#dc2626",
        Series::Duality => "#a855f7",
    }
}
