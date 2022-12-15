use crate::field::Field;

/// Represents a project in a prisma schema.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Project {
    /// Name of the project.
    pub name: String,
    /// Fields of the project.
    pub fields: Vec<Field>, // TODO: N/A
    /// Comments associated with this project.
    pub documentation: Option<String>,
}

impl Project {
    /// Creates a new project
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
            documentation: None,
        }
    }
}
