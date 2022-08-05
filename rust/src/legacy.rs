use sourcemap::{SourceMap, SourceView};
use std::{collections::HashMap, fs::read_to_string};

use crate::types::Frame;

pub struct Processor<'a> {
    pub sources: HashMap<String, SourceView<'a>>,
    pub sourcemaps: HashMap<String, SourceMap>,
    pub links: HashMap<String, String>,
    pub previous_frame_name: Option<String>,
}

impl<'a> Processor<'a> {
    pub fn new() -> Self {
        Processor {
            sources: HashMap::new(),
            sourcemaps: HashMap::new(),
            links: HashMap::new(),
            previous_frame_name: None,
        }
    }

    pub fn preprocess_step(&mut self, frames: &[Frame]) {
        self.populate_source_cache(frames);
    }
    pub fn process_frame(&mut self, frame: &mut Frame) {
        let sourcemap_url = self.links.get(&frame.abs_path).unwrap();
        let sourcemap_view = self.sourcemaps.get(sourcemap_url).unwrap();

        let minified_source = self.sources.get(&frame.abs_path).unwrap();

        let token = sourcemap_view
            .lookup_token(frame.lineno - 1, frame.colno - 1)
            .unwrap();

        // This is the place where we actually modify the frame
        frame.lineno = token.get_src_line() + 1;
        frame.colno = token.get_src_col() + 1;

        let token_name = token.get_name().map(|s| s.to_string());

        frame.function = minified_source
            .get_original_function_name(token, &frame.function)
            .unwrap_or_else(|| {
                if let Some(prev_frame_name) = self.previous_frame_name.as_ref() {
                    return prev_frame_name;
                }
                "<unknown>"
            })
            .to_string();

        self.previous_frame_name = token_name;

        let abs_path = token.get_source().unwrap().to_string();
        frame.abs_path = abs_path.clone();

        let unminified_source = self.sources.get(&abs_path).unwrap();

        self.expand_frame(frame, unminified_source);
    }

    fn populate_source_cache(&mut self, frames: &[Frame]) {
        for frame in frames {
            self.cache_source(frame.abs_path.to_string());
        }
    }

    fn cache_source(&mut self, abs_path: String) {
        let source_content = read_to_string(format!("src/fixtures/{}", abs_path)).unwrap();
        let source_view = SourceView::from_string(source_content);

        let sourcemap_ref = source_view.sourcemap_reference().unwrap().unwrap();
        let sourcemap_ref_url = sourcemap_ref.get_url();

        let sourcemap_content =
            read_to_string(format!("src/fixtures/{}", sourcemap_ref_url)).unwrap();
        let sourcemap = SourceMap::from_slice(sourcemap_content.as_bytes()).unwrap();

        let sm_sources = sourcemap.sources();
        for (idx, source) in sm_sources.enumerate() {
            let inline_source_content = sourcemap.get_source_view(idx as u32).unwrap().source();
            let inline_source_view = SourceView::from_string(inline_source_content.to_string());
            self.sources.insert(source.to_string(), inline_source_view);
        }

        self.sources.insert(abs_path.to_string(), source_view);
        self.sourcemaps
            .insert(sourcemap_ref_url.to_string(), sourcemap);
        self.links.insert(abs_path, sourcemap_ref_url.to_string());
    }

    fn expand_frame(&self, frame: &mut Frame, source: &SourceView) {
        let (pre, line, post) = get_source_context(source, frame.lineno as u32);

        frame.pre_context = pre;
        frame.context_line = line;
        frame.post_context = post;
    }
}

fn get_source_context(
    source: &SourceView,
    lineno: u32,
) -> (Option<String>, Option<String>, Option<String>) {
    (
        source.get_line(lineno - 2).map(String::from),
        source.get_line(lineno - 1).map(String::from),
        source.get_line(lineno).map(String::from),
    )
}
