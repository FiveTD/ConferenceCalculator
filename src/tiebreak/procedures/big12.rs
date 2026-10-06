/* Big 12 tiebreaker procedure.
 * Source: <TODO>
 */

use crate::tiebreak::{NotApplicableReason, Procedure, Reduction, RuleId, TiebreakRule, rules::*};

pub fn from_2024() -> Procedure {
    Procedure::new(rules(), rules()).with_reduction(Reduction::SeedLeader)
}

fn rules() -> Vec<Box<dyn TiebreakRule>> {
    vec![
        // (a) Head-to-head
        Box::new(HeadToHead {
            incomplete: IncompleteGroup::SeedUndefeated,
        }),
        // (b) Win % vs all common conference opponents
        Box::new(CommonOpponents { min_common: 1 }),
        // (c) Next-highest-placed common opponent, down the standings
        Box::new(StandingsWalk),
        // (d) Combined conference win % of conference opponents
        Box::new(OpponentsConferenceRecord),
        // (e) Total wins in a 12-game season, max one FCS win.
        Box::new(TotalWins),
        // (f) Highest SportSource Analytics rating (proprietary)
        Box::new(Unavailable {
            rule: RuleId::SportSourceRating,
            reason: NotApplicableReason::ExternalData,
        }),
        // (g) Coin toss
        Box::new(Unavailable {
            rule: RuleId::CoinToss,
            reason: NotApplicableReason::RandomDraw,
        }),
    ]
}
