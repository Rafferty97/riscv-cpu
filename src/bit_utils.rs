#[inline(always)]
pub fn extract(value: u32, start: u32, len: u32, pos: u32) -> u32 {
    ((value >> start) & (!0 >> (32 - len))) << pos
}
