#[derive(Debug)]
pub struct ProjectEvent {
    pub name: String,
    pub value: String,
}

impl ProjectEvent {
    pub fn report_function(&self) -> String {
        format!("report-{}", self.name)
    }
}
