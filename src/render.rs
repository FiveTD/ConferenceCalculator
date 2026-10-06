use crate::{
    model::{GameResult, TeamId},
    repository::ConferenceState,
    tiebreak::{
        Evidence, GameId, MetricValue, Placement, Record, Resolution, RuleOutcome, TieStatus,
        TraceStep,
    },
};

/// The one entry point: final order, then the trace as an indented tree.
pub fn print_resolution(state: &ConferenceState, resolution: &Resolution) {
    println!("Final order:");
    for p in &resolution.placements {
        print_placement(state, resolution, p);
    }

    println!("\nTrace:");
    for step in &resolution.trace {
        let indent = "  ".repeat(resolution.depth(step));
        println!("{indent}{}", step_line(state, step));
        print_step_evidence(state, &indent, step);
    }
}

fn print_placement(state: &ConferenceState, resolution: &Resolution, p: &Placement) {
    let tie = if p.status == TieStatus::Unresolved {
        " (tied)"
    } else {
        ""
    };
    let record = format_record(&state.conference_record(p.team));
    println!(
        "  {:>2}. {} ({}){tie}",
        p.place,
        name(state, &p.team),
        record
    );

    for step in resolution.steps_for(p) {
        let marker = if step.is_decisive() { "*" } else { " " };
        println!("      {marker} {}", step_line(state, step));
    }
}

fn step_line(state: &ConferenceState, step: &TraceStep) -> String {
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
    format!(
        "[{}] {:?} on ({}): {verdict}",
        step.id.0,
        step.rule,
        names(state, &step.tied)
    )
}

fn print_step_evidence(state: &ConferenceState, indent: &str, step: &TraceStep) {
    if let RuleOutcome::Separated { evidence, .. } | RuleOutcome::NoSeparation { evidence } =
        &step.outcome
    {
        print_evidence(state, indent, evidence);
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
        MetricValue::Record(r) => format_record(r),
    }
}

fn format_record(value: &Record) -> String {
    format!("{}-{}", value.wins, value.losses)
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
