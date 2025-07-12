// Configuration loading and struct (ShellConfig)
pub struct ShellConfig {
    pub prompt: String,
}

impl ShellConfig {
    pub fn load_from_file(_path: &str) -> Self {
        // TODO: Load config from file
        Self { prompt: "shellflow> ".to_string() }
    }
}
