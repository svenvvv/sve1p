use binrw::{BinRead, BinWrite};

use crate::utils::e1pstring::E1pString;

#[derive(BinRead, BinWrite, Debug)]
pub struct RoomIdent {
    pub name: E1pString<u32>,
    pub id: u32,
    pub is_string: u32,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct Room {
    pub name: E1pString<u32>,
    pub id: u16,
    ident_count: u32,
    #[br(count = ident_count)]
    pub idents: Vec<RoomIdent>,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct RoomTable {
    count: u16,
    #[br(count = count)]
    pub rooms: Vec<Room>,
}
