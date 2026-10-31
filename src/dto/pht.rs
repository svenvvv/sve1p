use binrw::{BinRead, BinWrite, helpers::until_eof};

#[derive(BinRead, BinWrite)]
pub struct PhraseTable {
    #[br(parse_with = until_eof)]
    pub indices: Vec<u32>,
}
