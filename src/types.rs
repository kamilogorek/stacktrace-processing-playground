#[derive(Debug, Default)]
pub struct Frame {
    pub abs_path: String,
    pub lineno: u32,
    pub colno: u32,
    pub context_line: Option<String>,
    pub pre_context: Option<String>,
    pub post_context: Option<String>,
    pub function: String,
}

#[derive(Debug)]
pub struct Stacktrace {
    pub frames: Vec<Frame>,
}
