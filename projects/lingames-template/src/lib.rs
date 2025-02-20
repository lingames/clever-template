#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod filters;
mod show_channels;
mod show_events;
mod show_templates;
mod show_versions;

pub use crate::{
    show_channels::ProjectChannel,
    show_events::ProjectEvent,
    show_templates::{ProjectData, TypescriptTracer},
    show_versions::ProjectVersion,
};
pub use askama::Template;
