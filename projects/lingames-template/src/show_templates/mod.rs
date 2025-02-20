use crate::{filters, show_channels::ProjectChannel, show_events::ProjectEvent, show_versions::ProjectVersion};
use askama::Template;

#[derive(Debug)]
pub struct ProjectData {
    pub project_name: String,
    pub channels: Vec<ProjectChannel>,
    pub versions: Vec<ProjectVersion>,
    pub events: Vec<ProjectEvent>,
}

impl ProjectData {
    pub fn tracer_type(&self) -> String {
        format!("{}-tracer", self.project_name)
    }
    pub fn channel_type(&self) -> String {
        format!("{}-channels", self.project_name)
    }
    pub fn version_type(&self) -> String {
        format!("{}-versions", self.project_name)
    }
    pub fn event_type(&self) -> String {
        format!("{}-events", self.project_name)
    }
}

#[derive(Template)]
#[template(path = "tracer.dejavu", escape = "txt")]
pub struct TypescriptTracer<'a> {
    pub data: &'a ProjectData,
}
