use std::{fs, path::Path};
use webx_api::{parse_project, parse_prose, Project, Prose};

fn main() {
    let prose_dir = Path::new("src/prose");
    let projects_dir = Path::new("src/projects");
    let mut proses: Vec<Prose> = Vec::new();

    for entry in fs::read_dir(prose_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().unwrap_or_default() == "org" {
            let prose = parse_prose(&path);
            if let Ok(prose) = prose {
                proses.push(prose);
            }
        }
    }

    let mut projects: Vec<Project> = Vec::new();

    for entry in fs::read_dir(projects_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().unwrap_or_default() == "org" {
            let project = parse_project(&path);
            if let Ok(project) = project {
                projects.push(project);
            }
        }
    }

    let prosejson = serde_json::to_string(&proses).unwrap();
    fs::write("src/prose.json", prosejson).unwrap();
    let projectsjson = serde_json::to_string(&projects).unwrap();
    fs::write("src/projects.json", projectsjson).unwrap();
}
