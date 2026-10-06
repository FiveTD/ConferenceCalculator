use crate::{
    model::{GameResult, TeamId},
    repository::ConferenceState,
    tiebreak::{Evidence, GameId, MetricValue, Resolution, RuleOutcome, TraceStep},
};

/// The one entry point: final order, then the trace as an indented tree.
pub fn print_resolution(state: &ConferenceState, resolution: &Resolution) {
    println!("Final order:");
    for (i, team) in resolution.order.iter().enumerate() {
        println!("  {:>2}. {}", i + 1, name(state, team));
    }

    println!("\nTrace:");
    for step in &resolution.trace {
        print_step(state, resolution, step);
    }
}

fn print_step(state: &ConferenceState, resolution: &Resolution, step: &TraceStep) {
    let indent = "  ".repeat(resolution.depth(step));

    let verdict = match &step.outcome {
        RuleOutcome::Separated { groups, .. } => format!(
            "split {}",
            groups
                .iter()
                .map(|g| format!("[{}]", names(state, g)))
                .collect::<Vec<_>>()
                .join(" > ")
        ),
        RuleOutcome::NoSeparation { .. } => "all equal".to_string(),
        RuleOutcome::NotApplicable { reason } => format!("n/a ({reason:?})"),
    };

    println!(
        "{indent}[{}] {:?} on ({}): {verdict}",
        step.id.0,
        step.rule,
        names(state, &step.tied),
    );

    // Only these two outcomes carry evidence.
    if let RuleOutcome::Separated { evidence, .. } | RuleOutcome::NoSeparation { evidence } =
        &step.outcome
    {
        print_evidence(state, &indent, evidence);
    }
}

fn print_evidence(state: &ConferenceState, indent: &str, evidence: &[Evidence]) {
    for e in evidence {
        let games: Vec<String> = e.games.iter().map(|&g| describe_game(state, g)).collect();
        println!(
            "{indent}    {} {}: {}",
            abbr(state, &e.team),
            format_metric(&e.value),
            games.join(", "),
        );
    }
}

/// The one place that has to learn about each new `MetricValue` variant.
fn format_metric(value: &MetricValue) -> String {
    match value {
        MetricValue::Record(r) => format!("{}-{}", r.wins, r.losses),
    }
}

fn describe_game(state: &ConferenceState, id: GameId) -> String {
    let Some(game) = state.game(id) else {
        return "?".into();
    };
    match &game.result {
        GameResult::Final {
            winner,
            home_points,
            away_points,
        } => {
            let (loser, winner_pts, loser_pts) = if *winner == game.home {
                (&game.away, home_points, away_points)
            } else {
                (&game.home, away_points, home_points)
            };
            format!(
                "{} {}-{} {}",
                abbr(state, winner),
                winner_pts,
                loser_pts,
                abbr(state, loser)
            )
        }
        GameResult::Scheduled => "scheduled".into(),
    }
}

fn name<'a>(state: &'a ConferenceState, team: &TeamId) -> &'a str {
    &state.teams[team].name
}

fn abbr<'a>(state: &'a ConferenceState, team: &TeamId) -> &'a str {
    &state.teams[team].abbreviation
}

fn names(state: &ConferenceState, teams: &[TeamId]) -> String {
    teams
        .iter()
        .map(|t| abbr(state, t))
        .collect::<Vec<_>>()
        .join(", ")
}
