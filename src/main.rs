mod legacy;
mod modern;
mod types;

use crate::legacy::Processor as LegacyProcessor;
use crate::modern::Processor as ModernProcessor;
use crate::types::Frame;
use crate::types::Stacktrace;

fn run_legacy() {
    let mut processor = LegacyProcessor::new();

    // let frames = vec![
    //     Frame {
    //         abs_path: String::from("bundle.js"),
    //         function: "n".to_string(),
    //         lineno: 1,
    //         colno: 20,
    //         ..Default::default()
    //     },
    //     Frame {
    //         abs_path: String::from("bundle.js"),
    //         function: "o".to_string(),
    //         lineno: 1,
    //         colno: 49,
    //         ..Default::default()
    //     },
    //     Frame {
    //         abs_path: String::from("bundle.js"),
    //         function: "Object.<anonymous>".to_string(),
    //         lineno: 1,
    //         colno: 53,
    //         ..Default::default()
    //     },
    // ];

    let frames = vec![
        Frame {
            abs_path: String::from("apply.js"),
            function: "i".to_string(),
            lineno: 1,
            colno: 48874,
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
            lineno: 1,
            colno: 59968,
            ..Default::default()
        },
    ];

    let mut stacktrace = Stacktrace { frames };

    // dbg!(&stacktrace);

    processor.preprocess_step(&stacktrace.frames);

    for frame in &mut stacktrace.frames {
        processor.process_frame(frame);
    }

    dbg!(&stacktrace);
}

fn run_modern() {
    let mut processor = ModernProcessor::new();

    // let frames = vec![
    //     Frame {
    //         abs_path: String::from("bundle.js"),
    //         function: "n".to_string(),
    //         lineno: 1,
    //         colno: 20,
    //         ..Default::default()
    //     },
    //     Frame {
    //         abs_path: String::from("bundle.js"),
    //         function: "o".to_string(),
    //         lineno: 1,
    //         colno: 49,
    //         ..Default::default()
    //     },
    //     Frame {
    //         abs_path: String::from("bundle.js"),
    //         function: "Object.<anonymous>".to_string(),
    //         lineno: 1,
    //         colno: 53,
    //         ..Default::default()
    //     },
    // ];

    let frames = vec![
        Frame {
            abs_path: String::from("apply.js"),
            function: "i".to_string(),
            lineno: 1,
            colno: 48874,
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
            lineno: 1,
            colno: 59968,
            ..Default::default()
        },
    ];
    let mut stacktrace = Stacktrace { frames };

    // dbg!(&stacktrace);

    // processor.preprocess_step(&stacktrace.frames);

    for frame in &mut stacktrace.frames {
        processor.process_frame(frame);
    }

    dbg!(&stacktrace);
}

fn main() {
    run_legacy();
    run_modern();
}
