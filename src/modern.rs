use sourcemap::{SourceMap, SourceView};
use std::{collections::HashMap, fs::read_to_string, ops::Deref};
use symbolic::iddqd::{ScopeLookupResult, SmCache, SmCacheWriter, SourceContext, SourcePosition};

use crate::types::Frame;

pub struct Processor<'a> {
    pub sources: HashMap<String, SourceView<'a>>,
    pub sourcemaps: HashMap<String, SourceMap>,
    pub links: HashMap<String, String>,
}

impl<'a> Processor<'a> {
    pub fn new() -> Self {
        Processor {
            sources: HashMap::new(),
            sourcemaps: HashMap::new(),
            links: HashMap::new(),
        }
    }

    pub fn preprocess_step(&mut self, frames: &[Frame]) {
        self.populate_source_cache(frames);
    }
    pub fn process_frame(&mut self, frame: &mut Frame) {
        // This is equivalent of `fetch_release_file` function
        let source_content = read_to_string(format!("src/fixtures/{}", &frame.abs_path)).unwrap();

        // This is equivalent of `discover_sourcemap` function
        let source_view = SourceView::from_string(source_content);
        let sourcemap_ref_url = source_view
            .sourcemap_reference()
            .unwrap()
            .unwrap()
            .get_url()
            .to_string();

        // This is equivalent of `fetch_sourcemap` or `fetch_release_file` + manual `SourceMapView` construction,
        // with base64 sourcemaps handling included.
        let sourcemap_content =
            read_to_string(format!("src/fixtures/{sourcemap_ref_url}")).unwrap();

        let writer = SmCacheWriter::new(source_view.source(), &sourcemap_content).unwrap();
        let mut buffer = Vec::new();
        writer.serialize(&mut buffer).unwrap();
        let cache = SmCache::parse(&buffer).unwrap();

        let sp = SourcePosition::new(frame.lineno - 1, frame.colno - 1);
        let token = cache.lookup(sp).unwrap();

        // This is the place where we actually modify the frame
        frame.lineno = token.line() + 1;
        frame.colno = 0; // todo!(); — columns not supported in iddqd currently
        frame.function = (match token.scope() {
            ScopeLookupResult::NamedScope(name) => name,
            ScopeLookupResult::AnonymousScope => "<anonymous>",
            ScopeLookupResult::Unknown => "<unknown>",
        })
        .to_string();

        let abs_path = token.file_name().unwrap();
        frame.abs_path = abs_path.to_string();

        let unminified_source = token.file().unwrap().source().unwrap();
        self.expand_frame(frame, unminified_source);
    }

    fn populate_source_cache(&mut self, frames: &[Frame]) {
        for frame in frames {
            self.cache_source(frame.abs_path.to_string());
        }
    }

    fn cache_source(&mut self, abs_path: String) {}

    fn expand_frame(&self, frame: &mut Frame, source: &str) {
        let (pre, line, post) = get_source_context(source, frame.lineno as usize);

        frame.pre_context = pre;
        frame.context_line = line;
        frame.post_context = post;
    }
}

fn get_source_context(
    source: &str,
    lineno: usize,
) -> (Option<String>, Option<String>, Option<String>) {
    let lines = source.lines().collect::<Vec<_>>();

    (
        lines.get(lineno - 2).map(|v| v.deref().to_string()),
        lines.get(lineno - 1).map(|v| v.deref().to_string()),
        lines.get(lineno).map(|v| v.deref().to_string()),
    )
}