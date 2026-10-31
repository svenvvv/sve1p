use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};

use crate::{
    dto::ast::{AmalgamationData, InlineStringData, Node, NodeVisitor},
    utils::{
        string::unescape_string,
        translation::{get_context, get_node_translation_context, hash_translation},
    },
};

pub struct TranslationReplacer {
    database: HashMap<u64, String>,
    current_translation_context: Option<String>,
    active_translation_context: Option<String>,
}

fn strip_quotes(string: &str) -> &str {
    if !string.starts_with('"') || !string.ends_with('"') {
        panic!("Expected quotes");
    }
    let mut ch = string.chars();
    ch.next();
    ch.next_back();
    ch.as_str()
}

impl TranslationReplacer {
    pub fn new(file: File) -> TranslationReplacer {
        let mut database = HashMap::new();

        for line_result in BufReader::new(file).lines() {
            let line = line_result.expect("Failed to read line from translation file");
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() != 3 {
                panic!(
                    "Expected 3 semicolon delimited fields, found {}.\nP.S.: don't use semicolons in translation texts.",
                    parts.len()
                );
            }

            let context = strip_quotes(parts[0]);
            let string = unescape_string(&strip_quotes(parts[1]));
            let hash = hash_translation(context, &string);

            let replacement = unescape_string(&strip_quotes(parts[2]));
            database.insert(hash, replacement);
        }

        TranslationReplacer {
            database,
            current_translation_context: None,
            active_translation_context: None,
        }
    }

    fn get_replacement(&self, string: &String) -> Option<&String> {
        let context = get_context(&self.active_translation_context);
        let hash = hash_translation(context, string);
        self.database.get(&hash)
    }
}

impl NodeVisitor for TranslationReplacer {
    fn visit_node(&mut self, node: &Node) -> Result<(), Error> {
        self.current_translation_context = get_node_translation_context(node);
        Ok(())
    }

    fn visit_amalgamation(&mut self, data: &mut AmalgamationData) -> Result<(), Error> {
        let mut set_context = false;
        if self.current_translation_context.is_some() {
            self.active_translation_context = self.current_translation_context.clone();
            set_context = true;
        }
        for c in data.children.iter_mut() {
            c.visit(self)?;
        }
        if set_context {
            self.active_translation_context = None;
        }
        Ok(())
    }

    fn visit_inline_string(&mut self, data: &mut InlineStringData) -> Result<(), Error> {
        if let Some(replacement) = self.get_replacement(&data.value.0) {
            println!(
                "Replaced string \"{}\" => \"{}\"",
                data.value.0, replacement
            );
            data.value.0 = replacement.clone();
        }
        Ok(())
    }
}
