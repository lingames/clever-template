use askama::Template;
use clever_sdk_template::{ProjectChannel, ProjectData, ProjectEvent, ProjectVersion, TypescriptTracer};
use std::{fs::File, io::Write};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample Data (replace with your actual data loading)
    let channels = vec![ProjectChannel { name: "wechat".to_string(), value: "hash-x".to_string() }];
    let channels = vec![ProjectChannel { name: "douyin".to_string(), value: "hash-y".to_string() }];
    let versions = vec![
        ProjectVersion { name: "v1_0_0".to_string(), value: "hash-xx".to_string() },
        ProjectVersion { name: "v1_1_0".to_string(), value: "hash-yy".to_string() },
        ProjectVersion { name: "v2_0_0".to_string(), value: "hash-zz".to_string() },
    ];
    let events = vec![
        ProjectEvent { name: "login-in".to_string(), value: "hash-xxx".to_string() },
        ProjectEvent { name: "recharge".to_string(), value: "hash-yyy".to_string() },
        ProjectEvent { name: "login-out".to_string(), value: "hash-zzz".to_string() },
    ];
    let tracer_data = ProjectData { channels, versions, events, project_name: "my-game".to_string() };

    // Create the templates and render it
    let template = TypescriptTracer { data: &tracer_data };

    let rendered_code = template.render()?;

    // Write the generated code to a file
    let mut file = File::create("tests/project_a_tracer.ts")?; // Output file name
    file.write_all(rendered_code.as_bytes())?;

    Ok(())
}
