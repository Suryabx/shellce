// In-memory variable store (VariableStore)
pub struct VariableStore {
    pub vars: std::collections::HashMap<String, String>,
}

impl VariableStore {
    pub fn new() -> Self {
        Self { vars: std::collections::HashMap::new() }
    }
    // TODO: Add methods for get/set/remove/list
}
