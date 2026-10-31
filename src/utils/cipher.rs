pub const INITIAL_E1P_KEY: EncryptionKey = EncryptionKey(0x82549171);

pub struct EncryptionKey(u32);

pub fn inplace(data: &mut Vec<u8>, key: EncryptionKey) {
    let mut u1: u32 = key.0 & 0xFFFF;
    let mut u2: u32 = key.0 >> 16;
    for i in 0..data.len() {
        let tmp: u32 = (u2 * u1) & 0xFFFFFFFF;
        let low16: u32 = tmp & 0xFFFF;
        let high16: u32 = (tmp >> 16) & 0xFFFF;
        u1 = ((low16.wrapping_sub(1)) & 0xFFFF) ^ high16;
        u2 = ((u1 >> 8) << 8) | data[i] as u32;
        u1 = (u1 & 0xFF) ^ u2;
        data[i] = (u1 & 0xFF) as u8;
    }
}
