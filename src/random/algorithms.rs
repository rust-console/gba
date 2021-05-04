
/// Advances a PCG with 32 bits of state.
#[macro_export]
macro_rules! pcg_core_state32 {
    ($state:expr, $inc:expr) => {
        $state.wrapping_mul(PCG_MULTIPLIER_32).wrapping_add($inc)
    };
}
#[macro_export]
macro_rules! rxs_m_xs_u32_to_u32 {
    ($state: expr) => {{
        $state ^= ($state >> (4 + ($state >> 28) as u32)).wrapping_mul(277803737u32);
        $state ^ ($state >> 22)
    }};
}
#[macro_export]
macro_rules! pcg_xsh_rr_u32_to_u16 {
    ($state: expr) => {
        ((($state ^ ($state >> 18)) >> 11) as u16).rotate_right($state >> 27) as u16 
    };
}
/// 29 = 32 - 3 (determine shift amount)
/// 6 = 16 - 3 - 7 ()
#[macro_export]
macro_rules! pcg_xsh_rs_u32_to_u16 {
    ($state: expr) => {
        (($state ^ ($state >> 6)) >> (6 + ($state >> 29))) as u16 
    };
}