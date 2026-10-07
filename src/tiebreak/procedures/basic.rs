use crate::tiebreak::{
    Procedure, TiebreakRule,
    rules::{CommonOpponents, HeadToHead, IncompleteGroup},
};

/// NOT any conference's official procedure. A stand-in so tests and examples
/// can run the machinery without a real rule set.
#[allow(dead_code)]
pub fn basic() -> Procedure {
    Procedure::new(rules(), rules())
}

// Rules aren't `Clone`, so build a fresh list for each side.
#[allow(dead_code)]
fn rules() -> Vec<Box<dyn TiebreakRule>> {
    vec![
        Box::new(HeadToHead {
            incomplete: IncompleteGroup::Skip,
        }),
        Box::new(CommonOpponents { min_common: 1 }),
    ]
}
