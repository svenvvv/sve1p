use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{Cursor, Seek},
    path::{Path, PathBuf},
};

use binrw::{BinReaderExt, BinWrite};

use crate::dto::{
    ast::{Node, NodeVariant, WriterContext},
    cht::{ConversationTable, ConversationType},
    r#gen::GenTable,
    pht::PhraseTable,
    rep::ReplacementTable,
    rht::RoomTable,
};

pub struct E1pModule {
    pub is_main: bool,
    pub conversations: ConversationTable,
    pub generated: GenTable,
    pub replacements: Option<ReplacementTable>,
    pub rooms: RoomTable,
    pub phrases: PhraseTable,
}

fn format_path(source: &Path, name_root: &str, ext: &str) -> PathBuf {
    let filename = format!("{}.{}", name_root, ext);
    return source.join(filename);
}

impl E1pModule {
    pub fn load_from_directory(source: &Path, name_root: &str, is_main: bool) -> E1pModule {
        let mut rep = None;
        let rht;
        let cht;
        let pht;
        let generated;

        if !is_main {
            let path = format_path(source, name_root, "rep");
            match fs::read(path) {
                Ok(data) => {
                    rep = Cursor::new(data)
                        .read_le()
                        .expect("Failed to parse rep file");
                }
                Err(_) => {
                    println!("No rep file found, skipping");
                }
            };
        }

        {
            let path = format_path(source, name_root, "rht");
            let data: Vec<u8> = fs::read(path).expect("Failed to read rht file");
            rht = Cursor::new(data)
                .read_le()
                .expect("Failed to parse rht file");
        }
        {
            let path = format_path(source, name_root, "cht");
            let data: Vec<u8> = fs::read(path).expect("Failed to read cht file");
            cht = Cursor::new(data)
                .read_le()
                .expect("Failed to parse cht file");
        }
        {
            let path = format_path(source, name_root, "pht");
            let data: Vec<u8> = fs::read(path).expect("Failed to read pht file");
            pht = Cursor::new(data)
                .read_le()
                .expect("Failed to parse pht file");
        }
        {
            let path = format_path(source, name_root, "gen");
            let data: Vec<u8> = fs::read(path).expect("Failed to read gen file");
            let data_size: u64 = data.len().try_into().expect("Failed to convert data len");
            let mut cursor = Cursor::new(data);
            generated = cursor
                .read_le_args((is_main,))
                .expect("Failed to parse gen file");

            let cursor_offset = cursor
                .stream_position()
                .expect("Failed to get stream position");
            let leftover_bytes = data_size - cursor_offset;
            if leftover_bytes != 0 {
                println!(
                    "Read cursor at 0x{:x}, left over {} bytes!",
                    cursor_offset, leftover_bytes
                );
            }
        }

        E1pModule {
            is_main,
            conversations: cht,
            generated,
            replacements: rep,
            phrases: pht,
            rooms: rht,
        }
    }

    pub fn write_gen(&mut self, path: &Path, context: &WriterContext) {
        let mut writer = Cursor::new(Vec::new());
        self.generated
            .write_le_args(&mut writer, (context.clone(), self.is_main))
            .expect("Failed to serialize gen file");
        fs::write(path, writer.into_inner()).expect("Failed to write gen file");
    }

    pub fn write_rep(&mut self, path: &Path) {
        let mut writer = Cursor::new(Vec::new());
        self.replacements
            .write_le(&mut writer)
            .expect("Failed to serialize rep file");
        fs::write(path, writer.into_inner()).expect("Failed to write rep file");
    }

    pub fn write_rht(&mut self, path: &Path) {
        let mut writer = Cursor::new(Vec::new());
        self.rooms
            .write_le(&mut writer)
            .expect("Failed to serialize rht file");
        fs::write(path, writer.into_inner()).expect("Failed to write rht file");
    }

    pub fn write_cht(&mut self, path: &Path) {
        let mut writer = Cursor::new(Vec::new());
        // We don't need to track relocations in CHT, just pass empty context
        self.conversations
            .write_le_args(&mut writer, (WriterContext::empty(),))
            .expect("Failed to serialize cht file");
        fs::write(path, writer.into_inner()).expect("Failed to write cht file");
    }

    pub fn write_pht(&mut self, path: &Path) {
        let mut writer = Cursor::new(Vec::new());
        self.phrases
            .write_le(&mut writer)
            .expect("Failed to serialize pht file");
        fs::write(path, writer.into_inner()).expect("Failed to write pht file");
    }

    fn collect_tracked_relocations(&self) -> HashSet<u32> {
        let mut addresses: HashSet<u32> = HashSet::new();

        for off in self.phrases.indices.iter() {
            addresses.insert(*off);
        }

        for conv in self.conversations.conversations.iter() {
            if conv.ctype != ConversationType::Conversation {
                continue;
            }
            addresses.insert(conv.filepos);
        }

        addresses
    }

    fn relocate_phrases(&mut self, relocations: &HashMap<u32, u32>) {
        for off in self.phrases.indices.iter_mut() {
            if let Some(new_off) = relocations.get(off) {
                *off = *new_off;
            }
        }

        for conv in self.conversations.conversations.iter_mut() {
            if conv.ctype != ConversationType::Conversation {
                continue;
            }
            if let Some(new_off) = relocations.get(&conv.filepos) {
                conv.filepos = *new_off;
            }
        }
    }

    pub fn write_to_directory(&mut self, source: &Path, name_root: &str) {
        // Write gen first to collect (if any) relocations
        let addresses = self.collect_tracked_relocations();
        let context = WriterContext::new(addresses);
        self.write_gen(&format_path(source, name_root, "gen"), &context);

        self.relocate_phrases(&context.relocatations.borrow());

        self.write_rep(&format_path(source, name_root, "rep"));
        self.write_rht(&format_path(source, name_root, "rht"));
        self.write_cht(&format_path(source, name_root, "cht"));
        self.write_pht(&format_path(source, name_root, "pht"));
    }

    pub fn get_ext_call_get_seq(&mut self) -> &mut Node {
        if !self.is_main {
            panic!("Not a main module, can't get external call get seq");
        }
        self.generated
            .root_nodes
            .first_mut()
            .expect("External call get seq not found")
    }

    pub fn get_ext_call_set_seq(&mut self) -> &mut Node {
        if !self.is_main {
            panic!("Not a main module, can't get external call set seq");
        }
        self.generated
            .root_nodes
            .get_mut(1)
            .expect("External call set seq not found")
    }

    pub fn get_root_seq_mut(&mut self) -> &mut Node {
        self.generated
            .root_nodes
            .last_mut()
            .expect("Root seq not found")
    }

    pub fn find_ctor(&mut self) -> (&Node, &Node) {
        let mut ctor = None;
        let mut dtor = None;

        let children;

        // children = &self.generated.root_nodes;
        if self.is_main {
            children = &self.generated.root_nodes;
        } else {
            let last = &self.generated.root_nodes.last().unwrap();
            children = match &last.variant {
                NodeVariant::Seq(data) => &data.children,
                _ => panic!("Unexpected root node variant: {:?}", last.variant),
            };
        }

        for node in children.iter() {
            match node.variant {
                NodeVariant::CtorPhrase(_) => {
                    if ctor.is_some() {
                        panic!("Found at least two ctors");
                    }
                    ctor = Some(node);
                }
                NodeVariant::DtorPhrase(_) => {
                    if dtor.is_some() {
                        panic!("Found at least two dtors");
                    }
                    dtor = Some(node);
                }
                _ => continue,
            }
        }

        if ctor.is_none() || dtor.is_none() {
            panic!("Failed to find ctor and dtor");
        }

        return (ctor.unwrap(), dtor.unwrap());
    }
}
