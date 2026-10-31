use std::hash::{DefaultHasher, Hash, Hasher};

use crate::dto::ast::{Node, NodeVariant};

pub fn get_node_translation_context(node: &Node) -> Option<String> {
    match &node.variant {
        NodeVariant::SpeakPhrase(_) => Some("speak_phrase".to_string()),
        NodeVariant::SetUserPhrase(_) => Some("user_phrase".to_string()),
        NodeVariant::OnTimerEvent(_) => Some("timer".to_string()),
        NodeVariant::UserFunc(n) => Some(format!(
            "user_func({})",
            n.x.as_ref().expect("Expected user func value"),
        )),
        NodeVariant::ThrowEvent(_) => Some("throw_event".to_string()),
        NodeVariant::Call(_) => Some("call".to_string()),
        NodeVariant::NewObjAttr(n) => n.string.as_ref().map(|v| format!("new_obj_attr({})", v)),
        NodeVariant::PushMember(n) => match n.string.as_ref() {
            Some(str) => {
                // The character gender checks happen inside other context nodes, we don't want to hide them
                if str.0 == "SEX" {
                    None
                } else {
                    Some(format!("push_member({})", str.0))
                }
            }
            None => None,
        },
        _ => None,
    }
}

pub fn get_context(context: &Option<String>) -> &str {
    match context {
        Some(v) => v,
        None => "none",
    }
}

pub fn hash_translation(context: &str, string: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    context.hash(&mut hasher);
    string.hash(&mut hasher);
    hasher.finish()
}
