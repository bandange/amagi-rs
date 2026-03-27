use crate::platforms::internal::random::PseudoRandom;

/// Generate the Bilibili `dm_img_inter` payload with browser-like defaults.
pub fn generate_dm_img_inter() -> String {
    let mut random = PseudoRandom::from_system();
    generate_dm_img_inter_with_values(
        1920,
        1080,
        10,
        10,
        random.next_mod(114),
        random.next_mod(514),
    )
}

/// Generate a deterministic Bilibili `dm_img_inter` payload.
pub fn generate_dm_img_inter_with_values(
    width: u32,
    height: u32,
    scroll_top: u32,
    scroll_left: u32,
    wh_random: u32,
    of_random: u32,
) -> String {
    let wh = [
        2 * width + 2 * height + 3 * wh_random,
        4 * width - height + wh_random,
        wh_random,
    ];
    let of = [
        3 * scroll_top + 2 * scroll_left + of_random,
        4 * scroll_top - 4 * scroll_left + 2 * of_random,
        of_random,
    ];

    format!(
        "{{\"ds\":[],\"wh\":[{},{},{}],\"of\":[{},{},{}]}}",
        wh[0], wh[1], wh[2], of[0], of[1], of[2]
    )
}
