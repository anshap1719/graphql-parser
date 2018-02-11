use std::fmt;

use ::format::{Displayable, Formatter, Style, format_directives};

use schema::ast::*;


impl Document {
    /// Format a document according to style
    pub fn format(&self, style: &Style) -> String {
        let mut formatter = Formatter::new(style);
        self.display(&mut formatter);
        formatter.into_string()
    }
}

fn to_string<T: Displayable>(v: &T) -> String {
    let style = Style::default();
    let mut formatter = Formatter::new(&style);
    v.display(&mut formatter);
    formatter.into_string()
}

fn description(description: &Option<String>, f: &mut Formatter) {
    if let Some(ref descr) = *description {
        f.indent();
        f.write_quoted(descr);
        f.endline();
    }
}


impl Displayable for Document {
    fn display(&self, f: &mut Formatter) {
        for item in &self.definitions {
            item.display(f);
        }
    }
}

impl Displayable for Definition {
    fn display(&self, f: &mut Formatter) {
        match *self {
            Definition::SchemaDefinition(ref s) => s.display(f),
            Definition::TypeDefinition(ref t) => t.display(f),
            Definition::TypeExtension(ref e) => e.display(f),
            Definition::DirectiveDefinition(ref d) => d.display(f),
        }
    }
}

impl Displayable for SchemaDefinition {
    fn display(&self, f: &mut Formatter) {
        f.indent();
        f.write("schema");
        format_directives(&self.directives, f);
        f.write(" ");
        f.start_block();
        if let Some(ref q) = self.query {
            f.indent();
            f.write("query: ");
            f.write(q);
            f.endline();
        }
        if let Some(ref m) = self.mutation {
            f.indent();
            f.write("mutation: ");
            f.write(m);
            f.endline();
        }
        if let Some(ref s) = self.subscription {
            f.indent();
            f.write("subscription: ");
            f.write(s);
            f.endline();
        }
        f.end_block();
    }
}

impl Displayable for TypeDefinition {
    fn display(&self, f: &mut Formatter) {
        match *self {
            TypeDefinition::Scalar(ref s) => s.display(f),
            TypeDefinition::Object(ref o) => o.display(f),
            TypeDefinition::Interface(ref i) => i.display(f),
            TypeDefinition::Union(ref u) => u.display(f),
            TypeDefinition::Enum(ref e) => e.display(f),
            TypeDefinition::InputObject(ref i) => i.display(f),
        }
    }
}

impl Displayable for ScalarType {
    fn display(&self, f: &mut Formatter) {
        description(&self.description, f);
        f.indent();
        f.write("scalar ");
        f.write(&self.name);
        format_directives(&self.directives, f);
        f.endline();
    }
}

impl Displayable for ScalarTypeExtension {
    fn display(&self, f: &mut Formatter) {
        unimplemented!();
    }
}

fn format_fields(fields: &[Field], f: &mut Formatter) {
    if !fields.is_empty() {
        f.write(" ");
        f.start_block();
        for fld in fields {
            fld.display(f);
        }
        f.end_block();
    } else {
        f.endline();
    }
}

impl Displayable for ObjectType {
    fn display(&self, f: &mut Formatter) {
        description(&self.description, f);
        f.indent();
        f.write("type ");
        f.write(&self.name);
        if !self.implements_interfaces.is_empty() {
            f.write(" implements ");
            f.write(&self.implements_interfaces[0]);
            for name in &self.implements_interfaces[1..] {
                f.write(" & ");
                f.write(name);
            }
        }
        format_directives(&self.directives, f);
        format_fields(&self.fields, f);
    }
}

impl Displayable for ObjectTypeExtension {
    fn display(&self, f: &mut Formatter) {
        f.indent();
        f.write("extend type ");
        f.write(&self.name);
        if !self.implements_interfaces.is_empty() {
            f.write(" implements ");
            f.write(&self.implements_interfaces[0]);
            for name in &self.implements_interfaces[1..] {
                f.write(" & ");
                f.write(name);
            }
        }
        format_directives(&self.directives, f);
        format_fields(&self.fields, f);
    }
}

impl Displayable for InputValue {
    fn display(&self, f: &mut Formatter) {
        if let Some(ref descr) = self.description {
            f.write_quoted(descr);
            f.write(" ");
        }
        f.write(&self.name);
        f.write(": ");
        self.value_type.display(f);
        if let Some(ref def) = self.default_value {
            f.write(" = ");
            def.display(f);
        }
        format_directives(&self.directives, f);
    }
}

fn format_arguments(arguments: &[InputValue], f: &mut Formatter) {
    if !arguments.is_empty() {
        f.write("(");
        arguments[0].display(f);
        for arg in &arguments[1..] {
            f.write(", ");
            arg.display(f);
        }
        f.write(")");
    }
}

impl Displayable for Field {
    fn display(&self, f: &mut Formatter) {
        description(&self.description, f);
        f.indent();
        f.write(&self.name);
        format_arguments(&self.arguments, f);
        f.write(": ");
        self.field_type.display(f);
        format_directives(&self.directives, f);
        f.endline();
    }
}

impl Displayable for InterfaceType {
    fn display(&self, f: &mut Formatter) {
        description(&self.description, f);
        f.indent();
        f.write("interface ");
        f.write(&self.name);
        format_directives(&self.directives, f);
        format_fields(&self.fields, f);
    }
}

impl Displayable for InterfaceTypeExtension {
    fn display(&self, f: &mut Formatter) {
        f.indent();
        f.write("extend interface ");
        f.write(&self.name);
        format_directives(&self.directives, f);
        format_fields(&self.fields, f);
    }
}

impl Displayable for UnionType {
    fn display(&self, f: &mut Formatter) {
        description(&self.description, f);
        f.indent();
        f.write("union ");
        f.write(&self.name);
        format_directives(&self.directives, f);
        f.write(" = ");
        if !self.types.is_empty() {
            f.write(&self.types[0]);
            for typ in &self.types[1..] {
                f.write(" | ");
                f.write(typ);
            }
        }
        f.endline();
    }
}

impl Displayable for UnionTypeExtension {
    fn display(&self, f: &mut Formatter) {
        unimplemented!();
    }
}

impl Displayable for EnumType {
    fn display(&self, f: &mut Formatter) {
        unimplemented!();
    }
}

impl Displayable for EnumTypeExtension {
    fn display(&self, f: &mut Formatter) {
        unimplemented!();
    }
}

impl Displayable for InputObjectType {
    fn display(&self, f: &mut Formatter) {
        unimplemented!();
    }
}

impl Displayable for InputObjectTypeExtension {
    fn display(&self, f: &mut Formatter) {
        unimplemented!();
    }
}

impl Displayable for TypeExtension {
    fn display(&self, f: &mut Formatter) {
        match *self {
            TypeExtension::Scalar(ref s) => s.display(f),
            TypeExtension::Object(ref o) => o.display(f),
            TypeExtension::Interface(ref i) => i.display(f),
            TypeExtension::Union(ref u) => u.display(f),
            TypeExtension::Enum(ref e) => e.display(f),
            TypeExtension::InputObject(ref i) => i.display(f),
        }
    }
}

impl Displayable for DirectiveDefinition {
    fn display(&self, f: &mut Formatter) {
        unimplemented!();
    }
}



macro_rules! impl_display {
    ($( $typ: ident, )+) => {
        $(
            impl fmt::Display for $typ {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.write_str(&to_string(self))
                }
            }
        )+
    };
}

impl_display!(
    Document,
    Definition,
    SchemaDefinition,
    TypeDefinition,
    TypeExtension,
    // ScalarType,
    // ScalarTypeExtension,
    // ObjectType,
    // ObjectTypeExtension,
    // Field,
    // InputValue,
    // InterfaceType,
    // InterfaceTypeExtension,
    // UnionType,
    // UnionTypeExtension,
    // EnumType,
    // EnumValue,
    // EnumTypeExtension,
    // InputObjectType,
    // InputObjectTypeExtension,
    // DirectiveLocation,
    // DirectiveDefinition,
);

