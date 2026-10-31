use std::{
    fs::File,
    io::{BufWriter, Error, Write},
};

use crate::{
    dto::{
        ast::{
            AmalgamationData, InlineIntData, InlineStringData, Node, NodeVisitor, NodeXValue,
            UnknownData,
        },
        cht::ConversationTable,
        e1pmodule::E1pModule,
        r#gen::GenTable,
        rht::RoomTable,
    },
    utils::{module_data::ModuleData, string::write_escaped_string},
};

pub struct SExpressionWriter {
    writer: BufWriter<File>,
    current_depth: usize,
    module_data: ModuleData,
}

impl SExpressionWriter {
    pub fn new(file: File, module: &E1pModule) -> SExpressionWriter {
        let writer = BufWriter::new(file);

        SExpressionWriter {
            writer,
            current_depth: 0,
            module_data: ModuleData::new(module),
        }
    }

    pub fn write(&mut self, module: &mut E1pModule) -> Result<(), Error> {
        self.writer.write("; Module info\n".as_bytes())?;
        self.write_module_info(&module.generated)?;
        self.writer.write("; Rooms table\n".as_bytes())?;
        self.write_rooms(&module.rooms)?;
        self.writer
            .write("; Conversation checklists table\n".as_bytes())?;
        self.write_checklists(&mut module.conversations)?;

        /*
        if module.is_main {
            self.writer.write("; Extern call get seq\n".as_bytes())?;
            module.get_ext_call_get_seq().visit(self)?;
            self.writer.write("; Extern call set seq\n".as_bytes())?;
            module.get_ext_call_set_seq().visit(self)?;
        }
        */

        for node in module.generated.root_nodes.iter_mut() {
            self.writer.write("; Root seq\n".as_bytes())?;
            node.visit(self)?;
        }

        Ok(())
    }

    fn write_checklists(&mut self, table: &mut ConversationTable) -> Result<(), Error> {
        write_node_start(&mut self.writer, "checklists")?;

        for (i, conv) in table.conversations.iter_mut().enumerate() {
            self.writer.write(
                format!("\n  ({} {} {} {} ", i, conv.id, conv.ctype, conv.filepos).as_bytes(),
            )?;

            self.current_depth = 2;
            conv.checklist.visit(self)?;
            self.current_depth = 0;

            write_node_end(&mut self.writer)?;
        }

        write_node_end(&mut self.writer)?;
        self.writer.write("\n".as_bytes())?;
        Ok(())
    }

    fn write_rooms(&mut self, table: &RoomTable) -> Result<(), Error> {
        write_node_start(&mut self.writer, "rooms")?;

        for room in table.rooms.iter() {
            self.writer
                .write(format!("\n  ({} \"{}\" (", room.id, room.name.0).as_bytes())?;

            for ident in room.idents.iter() {
                self.writer.write(
                    format!(
                        "\n    ({} \"{}\" {})",
                        ident.id, ident.name.0, ident.is_string
                    )
                    .as_bytes(),
                )?;
            }

            write_node_end(&mut self.writer)?;
            write_node_end(&mut self.writer)?;
        }

        write_node_end(&mut self.writer)?;
        self.writer.write("\n".as_bytes())?;
        Ok(())
    }

    fn write_module_info(&mut self, g: &GenTable) -> Result<(), Error> {
        write_node_start(&mut self.writer, "module")?;

        self.writer.write("\"".as_bytes())?;
        write_escaped_string(&mut self.writer, &g.name.0)?;
        self.writer.write("\" \"".as_bytes())?;
        write_escaped_string(&mut self.writer, &g.version.0)?;
        self.writer.write("\" ".as_bytes())?;

        // Just kind-of shoving these in here just-in-case.
        // I think we can generate these from the AST too, but I haven't checked for sure.
        self.writer.write(
            format!(
                "{} {} {} {} {}",
                g.flags_count,
                g.string_flags_count,
                g.objects_count,
                g.object_refs_count,
                g.list_count
            )
            .as_bytes(),
        )?;

        write_node_end(&mut self.writer)?;
        self.writer.write("\n".as_bytes())?;
        Ok(())
    }
}

fn write_node_start(writer: &mut BufWriter<File>, name: &str) -> Result<(), Error> {
    writer.write("(".as_bytes())?;
    writer.write(name.as_bytes())?;
    writer.write(" ".as_bytes())?;
    Ok(())
}

fn write_node_end(writer: &mut BufWriter<File>) -> Result<(), Error> {
    writer.write(")".as_bytes())?;
    Ok(())
}

impl<'a> NodeVisitor for SExpressionWriter {
    fn visit_node(&mut self, node: &Node) -> Result<(), Error> {
        let SExpressionWriter { module_data, .. } = self;

        if self.current_depth > 0 {
            self.writer.write("\n".as_bytes())?;
            self.writer
                .write("  ".repeat(self.current_depth).as_bytes())?;
        }

        write_node_start(&mut self.writer, node.variant.get_name())?;

        let node_off: u32 = node
            .opcode
            .pos
            .try_into()
            .expect("Failed to downcast offset");

        let mut replaced_phrase_ids = Vec::new();

        if let Some(phrase_indices) = module_data.phrase_indices_at_offset.get(&node_off) {
            write_node_start(&mut self.writer, "pht")?;

            for (i, phrase_index) in phrase_indices.iter().enumerate() {
                if i == 0 {
                    self.writer.write(format!("({}", phrase_index).as_bytes())?;
                } else {
                    self.writer.write(format!(" {}", phrase_index).as_bytes())?;
                }

                if let Some(old_id) = module_data.replacement_map.get(phrase_index) {
                    replaced_phrase_ids.push(old_id);
                }
            }

            write_node_end(&mut self.writer)?;
            write_node_end(&mut self.writer)?;
            self.writer.write(" ".as_bytes())?;
        }

        if replaced_phrase_ids.len() > 0 {
            write_node_start(&mut self.writer, "replaces")?;
            self.writer
                .write(format!("({}", replaced_phrase_ids[0]).as_bytes())?;

            for replaced_id in replaced_phrase_ids.iter().skip(1) {
                self.writer.write(format!(" {}", replaced_id).as_bytes())?;
            }

            write_node_end(&mut self.writer)?;
            write_node_end(&mut self.writer)?;
            self.writer.write(" ".as_bytes())?;
        }

        if let Some(conv_indices) = module_data.conversation_indices_at_offset.get(&node_off) {
            write_node_start(&mut self.writer, "checklist")?;
            self.writer
                .write(format!("({}", conv_indices[0]).as_bytes())?;

            for conv_id in conv_indices.iter().skip(1) {
                self.writer.write(format!(" {}", conv_id).as_bytes())?;
            }

            write_node_end(&mut self.writer)?;
            write_node_end(&mut self.writer)?;
            self.writer.write(" ".as_bytes())?;
        }

        Ok(())
    }

    fn visit_amalgamation(&mut self, data: &mut AmalgamationData) -> Result<(), Error> {
        let mut wrote_something = false;

        if data.x.is_some() {
            match data.x.as_ref().unwrap() {
                NodeXValue::Normal(x) => {
                    self.writer.write(x.to_string().as_bytes())?;
                }
                NodeXValue::Offset(x) => {
                    self.writer.write(format!("${}", x).as_bytes())?;
                }
            }
            wrote_something = true;
        }

        if data.string.is_some() {
            if wrote_something {
                self.writer.write(" ".as_bytes())?;
            }

            let str = data.string.as_ref().unwrap();
            self.writer.write("\"".as_bytes())?;
            write_escaped_string(&mut self.writer, &str.0)?;
            self.writer.write("\"".as_bytes())?;

            wrote_something = true;
        }

        if data.children.len() > 0 {
            if wrote_something {
                self.writer.write(" ".as_bytes())?;
            }

            self.current_depth += 1;
            for child in data.children.iter_mut() {
                child.visit(self)?;
            }
            self.current_depth -= 1;
        }

        write_node_end(&mut self.writer)?;
        Ok(())
    }

    fn visit_inline_int(&mut self, data: &mut InlineIntData) -> Result<(), Error> {
        self.writer.write(data.value.to_string().as_bytes())?;
        write_node_end(&mut self.writer)?;
        Ok(())
    }

    fn visit_inline_string(&mut self, data: &mut InlineStringData) -> Result<(), Error> {
        self.writer.write("\"".as_bytes())?;
        write_escaped_string(&mut self.writer, &data.value.0)?;
        self.writer.write("\"".as_bytes())?;
        write_node_end(&mut self.writer)?;
        Ok(())
    }

    fn visit_unknown(&mut self, data: &mut UnknownData) -> Result<(), Error> {
        self.writer.write(data.x.to_string().as_bytes())?;
        self.writer.write(" ".as_bytes())?;
        self.writer.write(data.y.to_string().as_bytes())?;
        write_node_end(&mut self.writer)?;
        Ok(())
    }
}
