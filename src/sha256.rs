
const H: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub fn sha256(message: String) -> String {
    let mut h = H;

    // 消息转换并填充
    let padded_message = process_message(message);
    // println!("{:?}",padded_message.len());
    // println!("{:?}",padded_message);
    
    // 消息分块
    let chunks = split_into_chunks(&padded_message);
    // println!("{:?}",chunks);

    // 处理每个块
    for chunk in chunks {
        // 每个块中的64轮 K(j)
        let w: [u32; 64] = extend_chunk(chunk);
        // println!("{:?}",w);
        let mut state = h.clone();
        // 64 轮哈希计算
        for i in 0..64 {
            let (_s1,_chh, temp1) = calculate_temp1(state[4], state[5], state[6], state[7], w[i], K[i]);
            let (_s0, _maj, temp2) = calculate_temp2(state[0], state[1], state[2]);

            // 更新哈希状态
            state[7] = state[6];
            state[6] = state[5];
            state[5] = state[4];
            state[4] = state[3].wrapping_add(temp1);
            state[3] = state[2];
            state[2] = state[1];
            state[1] = state[0];
            state[0] = temp1.wrapping_add(temp2);
        }

        // 相加更新 H(i)
        for i in 0..8 {
            h[i] = h[i].wrapping_add(state[i]);
        }
    }

    // 转换并返回结果
    convert_hash_to_string(h)
}

// convert message from string to vec<u8>
// pad 1&0 and len_message to n512bits
fn process_message(message: String) -> Vec<u8> {
    let mut message = message.into_bytes();

    let original_len_bits = ( message.len() as u64 ) * 8;

    message.push(0x80);
    while (message.len() % 64) != 56 {
        message.push(0);
    }
    message.extend_from_slice(&original_len_bits.to_be_bytes());

    message
}

// divide message by 512bits
fn split_into_chunks(padded_message:&[u8])-> Vec<&[u8]> {
    padded_message.chunks(64).collect()
}

// extend 16 words(32bits) to 64 words(32bits) for each chunk
fn extend_chunk(chunk: &[u8]) -> [u32;64] {
    let mut w = [0;64];
    for i in 0..16 {
        w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 +1], chunk[i * 4 + 2], chunk[i*4 +3]]);
    }
    for i in 16..64 {
        let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
    }
    w
}

fn calculate_temp1(e: u32, f: u32, g: u32, h: u32, w: u32, k: u32) -> (u32, u32, u32) {
    let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
    let ch = (e & f) ^ ((!e) & g);
    let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(k).wrapping_add(w);
    (s1, ch, temp1)
}

fn calculate_temp2(a: u32, b: u32, c: u32) -> (u32, u32, u32) {
    let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
    let maj = (a & b) ^ (a & c) ^ (b & c);
    let temp2 = s0.wrapping_add(maj);
    (s0, maj, temp2)
}

fn convert_hash_to_string(h: [u32; 8]) -> String {
    h.iter()
        .map(|&x| format!("{:08x}",x))  // 将 u32 转为 8位16进制字符串
        .collect::<Vec<String>>()  // 收集到 Vec 中
        .join("")  // 链接为字符串
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            sha256("abc".to_string()),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".to_string()
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            sha256("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".to_string()),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1".to_string()
        )
    }
}