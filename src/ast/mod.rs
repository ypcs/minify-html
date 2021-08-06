use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::from_utf8;

use crate::spec::tag::ns::Namespace;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ElementClosingTag {
    Omitted,
    Present,
    SelfClosing,
    Void,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ScriptOrStyleLang {
    CSS,
    Data,
    JS,
}

// Derive Eq for testing.
#[derive(Eq, PartialEq)]
pub enum NodeData {
    Bang {
        code: Vec<u8>,
        // If the source unexpectedly ended before `>`, we can't add it, as otherwise output could be longer than source.
        ended: bool,
    },
    Comment {
        code: Vec<u8>,
        // If the source unexpectedly ended before `-->`, we can't add it, as otherwise output could be longer than source.
        ended: bool,
    },
    Element {
        attributes: HashMap<Vec<u8>, Vec<u8>>,
        children: Vec<NodeData>,
        // If the source doesn't have a closing tag, then we can't add one, as otherwise output could be longer than source.
        closing_tag: ElementClosingTag,
        name: Vec<u8>,
        namespace: Namespace,
    },
    Instruction {
        code: Vec<u8>,
        // If the source unexpectedly ended before `?>`, we can't add it, as otherwise output could be longer than source.
        ended: bool,
    },
    // Entities should not be decoded in ScriptOrStyleContent.
    ScriptOrStyleContent {
        code: Vec<u8>,
        lang: ScriptOrStyleLang,
    },
    Text {
        value: Vec<u8>,
    },
}

fn str(bytes: &[u8]) -> &str {
    from_utf8(bytes).unwrap()
}

impl Debug for NodeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeData::Bang { code, ended } => f
                .debug_struct("Bang")
                .field("code", &from_utf8(code).unwrap().to_string())
                .field("ended", ended)
                .finish(),
            NodeData::Comment { code, ended } => f
                .debug_struct("Comment")
                .field("code", &from_utf8(code).unwrap().to_string())
                .field("ended", ended)
                .finish(),
            NodeData::Element {
                attributes,
                children,
                closing_tag,
                name,
                namespace,
            } => f
                .debug_struct("Element")
                .field("tag", &{
                    let mut out = format!("{:?}:{}", namespace, str(name));
                    for (n, v) in attributes {
                        out.push_str(format!(" {}={}", str(n), str(v)).as_str());
                    }
                    out
                })
                .field("children", children)
                .field("closing_tag", closing_tag)
                .finish(),
            NodeData::Instruction { code, ended } => f
                .debug_struct("Instruction")
                .field("code", &from_utf8(code).unwrap().to_string())
                .field("ended", ended)
                .finish(),
            NodeData::ScriptOrStyleContent { code, lang } => f
                .debug_struct("ScriptOrStyleContent")
                .field("code", &from_utf8(code).unwrap().to_string())
                .field("lang", lang)
                .finish(),
            NodeData::Text { value } => f.write_str(str(value)),
        }
    }
}
