
use bevy::prelude::*;

// Tests for Chronolog
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugins(TimeStructures{})
        .run();
}

// ################################################################################################# //
// HELPERS

/// Used to tell Bevy on .add_systems() calls which tests to run first.  When used in conjunction
/// with .chain().in_set(INSERT_ENUM_VALUE_HERE) on .add_systems() and with a followed .configure_sets()
/// call, it will force systems to run in sequential order even when they are split by different
/// .add_systems() calls. The reason why this exists is to get around Bevy's concurrent running of
/// add_system groups.
///
/// EXAMPLE
/// ```ignore
/// .add_systems(Startup, (
///     system_1,
///     system_2,
/// ).chain().in_set(TestSet::First))
///
/// .add_systems(Startup, (
///     system_3,
///     system_4,
/// ).chain().in_set(TestSet::Second))
///
/// .configure_sets(Startup, TestSet::First.before(TestSet::Second))
/// ```
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum TestSet {
    First,
    Second,
}

/// Used to label terminal output for tests into different colors.  Applied colors will tack onto
/// text until a different color is set.
///
/// ## EXAMPLE
/// ```ignore
/// println!("{}I am a message!{} Here are some words!", TestColors::PASS, TestColors::RESET);
/// ```
/// The words "I am a message!" will be whatever color is associated with PASS, and the string "Here
/// are some words!" will be whatever color is associated with RESET.
struct TestColors;
impl TestColors {
    const RESET: &'static str = "\x1b[0m";  // \x1b[0m  == WHATEVER THE DEFAULT COLOR IS FOR THE TERMINAL
    const FAIL:  &'static str = "\x1b[31m"; // \x1b[31m == RED TERMINAL TEXT
    const PASS:  &'static str = "\x1b[32m"; // \x1b[32m == GREEN TERMINAL TEXT
    const INFO:  &'static str = "\x1b[33m"; // \x1b[33m == YELLOW TERMINAL TEXT
}

/// Used for the [`check()`] function to print out a test message for the passed condition.
fn pass(test: &str) {
    println!("{}[PASS]{} {}", TestColors::PASS, TestColors::RESET, test);
}

/// Used for the [`check()`] function to print out a failed message for the passed condition.
fn fail(test: &str, reason: &str) {
    println!("{}[FAIL]{} {} — {}", TestColors::FAIL, TestColors::RESET, test, reason);
}

/// Used to determine if a test failed or passed based on the passed condition.  The supplied
/// reason is a failure message, not for if the test passed.
fn check(test: &str, condition: bool, reason: &str) {
    if condition {
        pass(test);
    }
    else {
        fail(test, reason);
    }
}
// ############################################################################################### //
