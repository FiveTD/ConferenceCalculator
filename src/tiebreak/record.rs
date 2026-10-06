use std::cmp::Ordering;

use crate::model::TeamId;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Record {
    pub wins: u8,
    pub losses: u8,
}

impl Record {
    pub fn games(self) -> u8 {
        self.wins + self.losses
    }

    /// Compare by winning percentage using integer cross-multiplication.
    /// A team with no games (0/0) counts as 0%.
    pub fn cmp_pct(self, other: Record) -> Ordering {
        let (aw, ag) = self.as_fraction();
        let (bw, bg) = other.as_fraction();
        (aw * bg).cmp(&(bw * ag))
    }

    /// Compare by number of wins.
    pub fn cmp_wins(self, other: Record) -> Ordering {
        self.wins.cmp(&other.wins)
    }

    /// Compare by number of losses (less is more).
    pub fn cmp_losses(self, other: Record) -> Ordering {
        other.losses.cmp(&self.losses)
    }

    /// Compare by standings order: win pct, then more wins, then fewer losses.
    pub fn cmp_standings(self, other: Record) -> Ordering {
        self.cmp_pct(other)
            .then_with(|| self.cmp_wins(other))
            .then_with(|| self.cmp_losses(other))
    }

    fn as_fraction(self) -> (u64, u64) {
        match self.games() {
            0 => (0, 1),
            g => (u64::from(self.wins), u64::from(g)),
        }
    }
}

/// Stably sort teams best-first by `key`, then group adjacent equal teams.
pub fn group_ranked<K: Copy>(
    mut items: Vec<(TeamId, K)>,
    cmp: impl Fn(K, K) -> Ordering,
) -> Vec<Vec<TeamId>> {
    items.sort_by(|a, b| cmp(b.1, a.1));

    let mut groups: Vec<Vec<TeamId>> = Vec::new();
    let mut last: Option<K> = None;
    for (team, key) in items {
        let tied_with_previous = last.is_some_and(|prev| cmp(prev, key) == Ordering::Equal);
        if tied_with_previous {
            groups
                .last_mut()
                .expect("a previous group exists")
                .push(team);
        } else {
            groups.push(vec![team]);
            last = Some(key)
        }
    }
    groups
}
