const INITIAL_STATE: [u32; 8] = [
    0x7380_166f,
    0x4914_b2b9,
    0x1724_42d7,
    0xda8a_0600,
    0xa96f_30bc,
    0x1631_38aa,
    0xe38d_ee4d,
    0xb0fb_0e4e,
];

pub(crate) fn sm3_sum(input: &[u8]) -> [u8; 32] {
    let mut message = input.to_vec();
    let bit_len = (message.len() as u64) * 8;

    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_len.to_be_bytes());

    let mut state = INITIAL_STATE;

    for chunk in message.chunks_exact(64) {
        let mut words = [0u32; 68];
        let mut expanded = [0u32; 64];

        for (index, word) in words.iter_mut().take(16).enumerate() {
            let offset = index * 4;
            *word = u32::from_be_bytes([
                chunk[offset],
                chunk[offset + 1],
                chunk[offset + 2],
                chunk[offset + 3],
            ]);
        }

        for index in 16..68 {
            let value = words[index - 16] ^ words[index - 9] ^ words[index - 3].rotate_left(15);
            words[index] = p1(value) ^ words[index - 13].rotate_left(7) ^ words[index - 6];
        }

        for index in 0..64 {
            expanded[index] = words[index] ^ words[index + 4];
        }

        let mut registers = state;

        for index in 0..64 {
            let ss1 = registers[0]
                .rotate_left(12)
                .wrapping_add(registers[4])
                .wrapping_add(t(index).rotate_left(index as u32))
                .rotate_left(7);
            let ss2 = ss1 ^ registers[0].rotate_left(12);
            let tt1 = ff(index, registers[0], registers[1], registers[2])
                .wrapping_add(registers[3])
                .wrapping_add(ss2)
                .wrapping_add(expanded[index]);
            let tt2 = gg(index, registers[4], registers[5], registers[6])
                .wrapping_add(registers[7])
                .wrapping_add(ss1)
                .wrapping_add(words[index]);

            registers[3] = registers[2];
            registers[2] = registers[1].rotate_left(9);
            registers[1] = registers[0];
            registers[0] = tt1;
            registers[7] = registers[6];
            registers[6] = registers[5].rotate_left(19);
            registers[5] = registers[4];
            registers[4] = p0(tt2);
        }

        for index in 0..8 {
            state[index] ^= registers[index];
        }
    }

    let mut output = [0u8; 32];
    for (index, register) in state.into_iter().enumerate() {
        output[index * 4..index * 4 + 4].copy_from_slice(&register.to_be_bytes());
    }
    output
}

fn ff(index: usize, x: u32, y: u32, z: u32) -> u32 {
    if index < 16 {
        x ^ y ^ z
    } else {
        (x & y) | (x & z) | (y & z)
    }
}

fn gg(index: usize, x: u32, y: u32, z: u32) -> u32 {
    if index < 16 {
        x ^ y ^ z
    } else {
        (x & y) | ((!x) & z)
    }
}

fn p0(value: u32) -> u32 {
    value ^ value.rotate_left(9) ^ value.rotate_left(17)
}

fn p1(value: u32) -> u32 {
    value ^ value.rotate_left(15) ^ value.rotate_left(23)
}

fn t(index: usize) -> u32 {
    if index < 16 { 0x79cc_4519 } else { 0x7a87_9d8a }
}
