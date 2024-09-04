library;

pub enum MatchResult {
    ZeroMatch: (),
    PartialMatch: (),
    FullMatch: (),
}

impl core::ops::Eq for MatchResult {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (Self::ZeroMatch, Self::ZeroMatch) => true,
            (Self::PartialMatch, Self::PartialMatch) => true,
            (Self::FullMatch, Self::FullMatch) => true,
            _ => false,
        }
    }
}
