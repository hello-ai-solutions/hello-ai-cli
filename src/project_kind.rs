#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectKind {
    Rust,
    Node,
    Python,
    Go,
    Java,
    Mixed,
}

pub fn detect_project_kind(cwd: &std::path::Path) -> ProjectKind {
    let p = |f: &str| cwd.join(f).exists();

    if p("Cargo.toml") {
        return ProjectKind::Rust;
    }
    if p("package.json") {
        return ProjectKind::Node;
    }
    if p("pyproject.toml") || p("requirements.txt") || p("setup.py") {
        return ProjectKind::Python;
    }
    if p("go.mod") {
        return ProjectKind::Go;
    }
    if p("pom.xml") {
        return ProjectKind::Java;
    }
    ProjectKind::Mixed
}
