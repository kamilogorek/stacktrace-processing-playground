mod legacy;
mod modern;
mod types;

use crate::legacy::Processor as LegacyProcessor;
use crate::modern::Processor as ModernProcessor;
use crate::types::Frame;
use crate::types::Stacktrace;

#[allow(dead_code)]
fn frames_bundle() -> Vec<Frame> {
    vec![
        Frame {
            abs_path: String::from("bundle.js"),
            function: "n".to_string(),
            lineno: 1,
            colno: 20,
            ..Default::default()
        },
        Frame {
            abs_path: String::from("bundle.js"),
            function: "o".to_string(),
            lineno: 1,
            colno: 49,
            ..Default::default()
        },
        Frame {
            abs_path: String::from("bundle.js"),
            function: "Object.<anonymous>".to_string(),
            lineno: 1,
            colno: 53,
            ..Default::default()
        },
    ]
}

#[allow(dead_code)]
fn frames_apply() -> Vec<Frame> {
    vec![
        Frame {
            abs_path: String::from("apply.js"),
            lineno: 1,
            colno: 59968,
            ..Default::default()
        },
        Frame {
            abs_path: String::from("apply.js"),
            lineno: 1,
            colno: 59982,
            ..Default::default()
        },
        Frame {
            abs_path: String::from("apply.js"),
            function: "i".to_string(),
            lineno: 1,
            colno: 48874,
            ..Default::default()
        },
    ]
}

#[allow(dead_code)]
fn frames_module() -> Vec<Frame> {
    vec![
        Frame {
            abs_path: String::from("module.js"),
            function: "HTMLButtonElement.t".to_string(),
            lineno: 1,
            colno: 63,
            ..Default::default()
        },
        Frame {
            abs_path: String::from("module.js"),
            function: "n".to_string(),
            lineno: 1,
            colno: 47,
            ..Default::default()
        },
        Frame {
            abs_path: String::from("module.js"),
            lineno: 1,
            colno: 34,
            ..Default::default()
        },
    ]
}

#[allow(dead_code)]
fn frames_sentry() -> Vec<Frame> {
    vec![
        Frame {
            abs_path: String::from("sentry.js"),
            function: "HTMLButtonElement.i".to_string(),
            lineno: 1,
            colno: 51239,
            ..Default::default()
        },
        Frame {
            abs_path: String::from("sentry.js"),
            function: "HTMLButtonElement.ln".to_string(),
            lineno: 1,
            colno: 60099,
            ..Default::default()
        },
        Frame {
            abs_path: String::from("sentry.js"),
            function: "dn".to_string(),
            lineno: 1,
            colno: 58944,
            ..Default::default()
        },
        Frame {
            abs_path: String::from("sentry.js"),
            lineno: 1,
            colno: 58931,
            ..Default::default()
        },
    ]
}

fn run_legacy() {
    let mut processor = LegacyProcessor::new();
    let frames = frames_sentry();
    let mut stacktrace = Stacktrace { frames };

    processor.preprocess_step(&stacktrace.frames);

    for frame in &mut stacktrace.frames {
        processor.process_frame(frame);
    }

    for frame in &mut stacktrace.frames {
        dbg!(&frame);
    }
}

fn run_modern() {
    let mut processor = ModernProcessor::new();
    let frames = frames_sentry();
    let mut stacktrace = Stacktrace { frames };

    for frame in &mut stacktrace.frames {
        processor.process_frame(frame);
    }

    for frame in &mut stacktrace.frames {
        dbg!(&frame);
    }
}

fn main() {
    // (sentry.js)
    //
    // Same behavior as in sentry monolith - fn names read using previous tokens if unknown (simple heuristics)
    //
    // <unknown>
    // apply
    // bar
    // foo
    println!("\nLegacy monolith resolution:");
    run_legacy();

    // (sentry.js)
    //
    // Without heuristics:
    //
    // sentryWrapped
    // buttonCallback
    // bar
    // <unknown>
    //
    // With heuristics:
    //
    // sentryWrapped
    // buttonCallback
    // bar
    // foo <= this function name is read from the previous token's call-site function name
    println!("\nModern smcache resolution:");
    run_modern();
    println!();
}
