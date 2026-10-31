use std::fmt;

use binrw::{BinRead, BinWrite};

use crate::dto::ast::{Node, WriterContext};

#[derive(BinRead, BinWrite, Debug, PartialEq)]
pub enum ConversationType {
    #[brw(magic(0u8))]
    Conversation,
    #[brw(magic(1u8))]
    Room,
}

impl fmt::Display for ConversationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ConversationType::Conversation => "conversation",
            ConversationType::Room => "room",
        };
        write!(f, "{}", value)
    }
}

#[derive(BinRead, BinWrite, Debug)]
#[bw(import(ctx: WriterContext))]
pub struct Conversation {
    pub id: u16,
    pub filepos: u32,
    pub ctype: ConversationType,
    #[bw(args(ctx.clone()))]
    pub checklist: Node,
}

#[derive(BinRead, BinWrite, Debug)]
#[bw(import(ctx: WriterContext))]
pub struct ConversationTable {
    count: u16,
    #[br(count = count)]
    #[bw(args(ctx.clone()))]
    pub conversations: Vec<Conversation>,
}
