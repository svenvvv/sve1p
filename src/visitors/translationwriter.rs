use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Error, Write},
};

use crate::{
    dto::ast::{AmalgamationData, InlineStringData, Node, NodeVisitor},
    utils::{
        string::write_escaped_string,
        translation::{get_context, get_node_translation_context, hash_translation},
    },
};

pub struct TranslationWriter {
    writer: BufWriter<File>,
    visited_strings: HashSet<u64>,
    current_translation_context: Option<String>,
    active_translation_context: Option<String>,
}

impl TranslationWriter {
    pub fn new(file: File) -> TranslationWriter {
        let writer = BufWriter::new(file);
        TranslationWriter {
            writer,
            visited_strings: HashSet::new(),
            current_translation_context: None,
            active_translation_context: None,
        }
    }

    fn write_string(&mut self, string: &String) -> Result<(), Error> {
        if string.len() == 0 {
            return Ok(());
        }

        let context = get_context(&self.active_translation_context);
        let hash = hash_translation(context, string);

        if self.visited_strings.contains(&hash) {
            println!("Skipping already visited string \"{}\"", string);
            return Ok(());
        }
        self.visited_strings.insert(hash);

        println!(
            "Writing translation string (ctx=\"{}\"): \"{}\"",
            context, string
        );

        self.writer.write("\"".as_bytes())?;
        self.writer.write(context.as_bytes())?;
        self.writer.write("\";\"".as_bytes())?;
        write_escaped_string(&mut self.writer, string)?;
        self.writer.write("\";\"".as_bytes())?;
        write_escaped_string(&mut self.writer, string)?;
        self.writer.write("\"\n".as_bytes())?;

        Ok(())
    }
}

impl NodeVisitor for TranslationWriter {
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
        self.write_string(&data.value.0)?;
        Ok(())
    }
}
