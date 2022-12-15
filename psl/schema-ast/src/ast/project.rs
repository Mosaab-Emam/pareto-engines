use super::{Attribute, Comment, Field, Identifier, Span, WithAttributes, WithDocumentation, WithIdentifier, WithSpan};

/// An opaque identifier for a field in an AST model. Use the
/// `model[field_id]` syntax to resolve the id to an `ast::Field`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldId(pub(super) u32);

impl FieldId {
    /// Used for range bounds when iterating over BTreeMaps.
    pub const MIN: FieldId = FieldId(0);
    /// Used for range bounds when iterating over BTreeMaps.
    pub const MAX: FieldId = FieldId(u32::MAX);
}

impl std::ops::Index<FieldId> for Project {
    type Output = Field;

    fn index(&self, index: FieldId) -> &Self::Output {
        &self.fields[index.0 as usize]
    }
}

/// A project declaration.
#[derive(Debug, Clone)]
pub struct Project {
    /// The name of the project.
    ///
    /// ```ignore
    /// project foo { .. }
    ///         ^^^
    /// ```
    pub(crate) name: Identifier,

    /// The fields of the project.
    ///
    /// ```ignore
    /// project foo {
    ///   backend server
    ///   ^^^^^^^^^^^^^^^^
    ///   field String
    ///   ^^^^^^^^^^^^
    /// }
    /// ```
    pub(crate) fields: Vec<Field>,

    /// The attributes of this project.
    ///
    /// ```ignore
    /// project foo {
    ///   backend server
    ///   field String
    ///
    ///   @@attr1()
    ///   ^^^^^^^^^^^^^^^^
    ///   @@attr2()
    ///   ^^^^^^^^^^^^
    /// }
    /// ```
    pub attributes: Vec<Attribute>,

    /// The documentation of this project.
    ///
    /// ```ignore
    /// /// Lorem ipsum
    ///     ^^^^^^^^^^^
    /// project foo {
    ///   backend server
    ///   field String
    /// }
    /// ```
    pub(crate) documentation: Option<Comment>,

    /// The location of this project in the text representation.
    pub(crate) span: Span,
}

impl Project {
    pub fn iter_fields(&self) -> impl ExactSizeIterator<Item = (FieldId, &Field)> {
        self.fields
            .iter()
            .enumerate()
            .map(|(idx, field)| (FieldId(idx as u32), field))
    }
}

impl WithIdentifier for Project {
    fn identifier(&self) -> &Identifier {
        &self.name
    }
}

impl WithSpan for Project {
    fn span(&self) -> Span {
        self.span
    }
}

impl WithAttributes for Project {
    fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl WithDocumentation for Project {
    fn documentation(&self) -> Option<&str> {
        self.documentation.as_ref().map(|doc| doc.text.as_str())
    }
}
