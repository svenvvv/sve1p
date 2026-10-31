use binrw::{BinRead, BinWrite, helpers::until_eof};

use crate::{
    dto::ast::{Node, WriterContext},
    utils::e1pstring::E1pString,
};

#[derive(BinRead, BinWrite)]
#[bw(import(ctx: WriterContext, is_main: bool))]
#[br(import(is_main: bool))]
pub struct GenTable {
    pub flags_count: u32,
    pub string_flags_count: u32,
    pub objects_count: u32,
    pub object_refs_count: u32,
    pub list_count: u32,

    #[br(if(!is_main, E1pString::from("MAIN")))]
    #[bw(if(!is_main))]
    pub name: E1pString<u32, false>,
    #[br(if(!is_main, E1pString::from("0.0")))]
    #[bw(if(!is_main))]
    pub version: E1pString<u32, false>,

    #[bw(args(ctx.clone()))]
    // #[br(count = if is_main { 3 } else { 1 })]
    #[br(parse_with = until_eof)]
    pub root_nodes: Vec<Node>,
}
