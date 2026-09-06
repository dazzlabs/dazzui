//! The React story list is the contract's index.
//!
//! `../tests/parity/stories.json` is what `apps/storybook` renders, written
//! by its parity extractor and distributed into this package; every id in it
//! is a story this playground is expected to draw under the same name. A
//! story missing here is a gap in the port, and a story here that React does
//! not have is a name the port invented — both are the kind of drift that
//! used to be invisible.
//!
//! The two lists beside it are the ratchet: `pending.txt` names the React
//! stories not drawn yet, `extra.txt` the ones drawn here that React has no
//! counterpart for. Each is held both ways, so an entry cannot go stale — a
//! pending story that lands has to leave the list in the same change.
//!
//! None of this is committed: the data is what `pnpm parity` writes, and the
//! lists are recorded by the first run that finds them missing. A checkout
//! without the data skips rather than fails.

use std::collections::BTreeSet;

use crate::storybook::story_ids;

/// Where the parity data this package was given lives, relative to the
/// playground. It belongs to `packages/ui_gpui` rather than to either crate
/// inside it, since the library's own parity test reads the same copy.
fn parity(file: &str) -> String {
    format!("{}/../tests/parity/{file}", env!("CARGO_MANIFEST_DIR"))
}

fn listed(file: &str) -> Option<BTreeSet<String>> {
    std::fs::read_to_string(parity(file)).ok().map(|text| {
        text.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(str::to_string)
            .collect()
    })
}

/// The list as it stands, or — on a first run — the list as it is now,
/// written down so the next run is held to it.
fn ratchet(file: &str, note: &str, current: &BTreeSet<String>) -> BTreeSet<String> {
    if let Some(existing) = listed(file) {
        return existing;
    }
    let mut lines = vec![
        format!("# {note}"),
        "# Recorded by the first test run; held both ways from now on, so it".to_string(),
        "# can only shrink. Shrink it; never grow it to pass.".to_string(),
    ];
    lines.extend(current.iter().cloned());
    lines.push(String::new());
    std::fs::write(parity(file), lines.join("\n")).unwrap();
    println!("recorded {} entries into {file}", current.len());
    current.clone()
}

#[test]
fn every_story_id_is_unique() {
    let ids = story_ids();
    let unique: BTreeSet<&String> = ids.iter().collect();
    assert_eq!(unique.len(), ids.len(), "two stories share an id");
}

#[test]
fn every_react_story_is_drawn_here_or_listed_as_pending() {
    let Ok(manifest) = std::fs::read_to_string(parity("stories.json")) else {
        println!("no tests/parity/stories.json — run `pnpm parity` at the root; skipped");
        return;
    };
    let manifest: serde_json::Value = serde_json::from_str(&manifest).unwrap();
    let react: BTreeSet<String> = manifest["stories"]
        .as_array()
        .unwrap()
        .iter()
        .map(|story| story["id"].as_str().unwrap().to_string())
        .collect();
    let drawn: BTreeSet<String> = story_ids().into_iter().collect();

    let missing: BTreeSet<String> = react.difference(&drawn).cloned().collect();
    let surplus: BTreeSet<String> = drawn.difference(&react).cloned().collect();
    let pending = ratchet(
        "pending.txt",
        "React stories this playground does not draw yet, one id per line.",
        &missing,
    );
    let extra = ratchet(
        "extra.txt",
        "Stories this playground draws that React has no counterpart for.",
        &surplus,
    );

    let mut problems: Vec<String> = Vec::new();
    for id in missing.difference(&pending) {
        problems.push(format!(
            "React draws {id} and this playground does not — draw it, or add it to pending.txt"
        ));
    }
    for id in pending.difference(&missing) {
        problems.push(format!(
            "{id} is in pending.txt but is drawn now, or gone from React — remove it"
        ));
    }
    for id in surplus.difference(&extra) {
        problems.push(format!(
            "{id} is drawn here but is not a React story — rename it, or add it to extra.txt"
        ));
    }
    for id in extra.difference(&surplus) {
        problems.push(format!(
            "{id} is in extra.txt but is no longer drawn — remove it"
        ));
    }
    problems.sort();

    for problem in &problems {
        println!("{problem}");
    }
    assert!(
        problems.is_empty(),
        "{} story ids are out of step with the lists beside them; see above",
        problems.len()
    );
}
