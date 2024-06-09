use std::collections::HashMap;

#[derive(Default)]
pub struct BaseSettings {
    opts: HashMap<String, String>,
    cli_opts: HashMap<String, String>,
}

impl BaseSettings {
    pub fn new() -> BaseSettings {
        BaseSettings::default()
    }

    // other methods go here...
}