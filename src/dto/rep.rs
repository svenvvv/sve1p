use binrw::{BinRead, BinWrite, helpers::until_eof};

#[derive(BinRead, BinWrite)]
pub struct Replacement {
    pub old_id: u32,
    pub new_id: u32,
}

#[derive(BinRead, BinWrite)]
pub struct ReplacementTable {
    #[br(parse_with = until_eof)]
    pub replacements: Vec<Replacement>,
}
