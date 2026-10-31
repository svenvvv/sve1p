use std::collections::HashMap;

use crate::dto::{cht::ConversationType, e1pmodule::E1pModule};

pub struct ModuleData {
    pub conversation_indices_at_offset: HashMap<u32, Vec<u32>>,
    pub phrase_indices_at_offset: HashMap<u32, Vec<u32>>,
    /// Map of new-old phrase IDs.
    /// New ID is from this module and the old from the base game.
    pub replacement_map: HashMap<u32, u32>,
}

impl ModuleData {
    pub fn new(module: &E1pModule) -> ModuleData {
        let mut conversation_indices_at_offset: HashMap<u32, Vec<u32>> = HashMap::new();
        for (big_i, conv) in module.conversations.conversations.iter().enumerate() {
            if conv.ctype == ConversationType::Room {
                // Rooms store some other data in the filepos, possibly RHT indices?
                continue;
            }
            let i: u32 = big_i.try_into().expect("Failed to downcast index");
            if let Some(arr) = conversation_indices_at_offset.get_mut(&conv.filepos) {
                arr.push(i);
            } else {
                conversation_indices_at_offset.insert(conv.filepos, vec![i]);
            }
        }

        let mut phrase_indices_at_offset: HashMap<u32, Vec<u32>> = HashMap::new();
        for (big_i, offset) in module.phrases.indices.iter().enumerate() {
            let i: u32 = big_i.try_into().expect("Failed to downcast index");
            if let Some(arr) = phrase_indices_at_offset.get_mut(offset) {
                arr.push(i);
            } else {
                phrase_indices_at_offset.insert(*offset, vec![i]);
            }
        }

        let replacement_map = match module.replacements.as_ref() {
            Some(v) => v
                .replacements
                .iter()
                .map(|r| (r.new_id, r.old_id))
                .collect(),
            None => HashMap::new(),
        };

        ModuleData {
            conversation_indices_at_offset,
            phrase_indices_at_offset,
            replacement_map,
        }
    }
}
