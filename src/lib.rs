use std::sync::Arc;

// namespace functions.
fn get_buttons() -> Vec<Arc<dyn Button>> {
    vec![Arc::new(StopButton {}), Arc::new(GoButton {})]
}

fn press(button: Arc<dyn Button>) -> Arc<dyn Button> {
    button
}

pub trait Button: Send + Sync {
    fn name(&self) -> String;
}

struct GoButton {}

impl Button for GoButton {
    fn name(&self) -> String {
        "go".to_string()
    }
}

struct StopButton {}

impl Button for StopButton {
    fn name(&self) -> String {
        "stop".to_string()
    }
}

uniffi::include_scaffolding!("lib");