use rand::rngs::OsRng;
use rand::RngCore;
use std::str;

const S_BOX: [[u8; 16]; 16] = [
    [0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76],
    [0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0],
    [0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15],
    [0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75],
    [0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84],
    [0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf],
    [0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8],
    [0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2],
    [0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73],
    [0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb],
    [0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79],
    [0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08],
    [0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a],
    [0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e],
    [0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf],
    [0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16],
];

const RCON: [u8; 14] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36, 0x6c, 0xd8, 0xab, 0x4d];


fn generate_aes_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

fn rot_word(word: [u8; 4]) -> [u8; 4]{
    return [word[1], word[2], word[3], word[0]]
}

fn sub_word(mut word: [u8; 4]) -> [u8; 4]{
    for byte in word.iter_mut(){
        *byte = find_s_box_sub(*byte)
    }
    return word;
}

fn xor_words(a: [u8; 4], b: [u8; 4]) -> [u8; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}

fn key_expansion(key: [u8; 32]) -> [[[u8; 4]; 4]; 15] {
    let mut round_keys = [[0u8; 4]; 60];

    for i in 0..8 {
        round_keys[i] = [key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]];

    }

    for i in 8..60 {
        let mut temp = round_keys[i];

        if i % 8 == 0{

            let mut rcon = [0u8;4];
            rcon[0] = RCON[(i/8) - 1];
            temp = xor_words(round_keys[i - 8], sub_word(rot_word(round_keys[i - 1]))) ;
            temp[0] = temp[0] ^ RCON[(i/8) - 1];
            round_keys[i] = temp;

        }
        else if i % 8 == 4{
            round_keys[i] = xor_words(sub_word(round_keys[i - 1]), round_keys[i - 8]);
        }
        else{
            round_keys[i] = xor_words(round_keys[i - 8], round_keys[i - 1]);
        }
        
    }

    let mut expanded_states = [[[0u8; 4]; 4]; 15];

    expanded_states[0] = [round_keys[0], round_keys[1], round_keys[2],round_keys[3]];

    for i in 1..15 {
        expanded_states[i] = [round_keys[i * 4], round_keys[i * 4 + 1], round_keys[i * 4 + 2 ],round_keys[i * 4 + 3]];
    }

    return expanded_states;
}


fn find_s_box_sub(byte: u8) -> u8{
    let row =  ((byte >> 4) & 0xF) as usize;
    let col = (byte & 0xF) as usize;
    return S_BOX[row][col];

}

fn sub_bytes(mut state: [[u8; 4]; 4]) -> [[u8; 4]; 4]{
    for row in state.iter_mut() {
        for byte in row.iter_mut() {
            *byte = find_s_box_sub(*byte);
        }
    }
    return state;
}

fn rotate_row(mut row: [u8; 4], n: usize) -> [u8; 4]{
    row.rotate_left(n);
    return row;
}

fn _shift_row(mut state: [[u8; 4]; 4]) -> [[u8; 4]; 4]{
    for i in 1..4 {
        state[i] = rotate_row(state[i], i);
    }
    return state;
}

fn shift_row(mut state: [[u8; 4]; 4]) -> [[u8; 4]; 4]{
    let temp_state = state.clone();

    state[0][1] = temp_state[1][1];
    state[1][1] = temp_state[2][1];
    state[2][1] = temp_state[3][1];
    state[3][1] = temp_state[0][1];

    let temp_state = state.clone();
    state[0][2] = temp_state[2][2];
    state[1][2] = temp_state[3][2];
    state[2][2] = temp_state[0][2];
    state[3][2] = temp_state[1][2];

    let temp_state = state.clone();
    state[0][3] = temp_state[3][3];
    state[1][3] = temp_state[0][3];
    state[2][3] = temp_state[1][3];
    state[3][3] = temp_state[2][3];

    return state;
}

fn print_state(state: [[u8; 4]; 4]){
    for row in state.iter() {
        println!("{:?}", row);
    }
    println!()
}

fn print_state_hex(state: [[u8; 4]; 4]) {
    for row in state.iter() {
        for &value in row.iter() {
            print!("{:02x} ", value);
        }
        println!();
    }
    println!();
}


fn double_in_galois_field(mut num : u8) -> u8{
    let msb = num & 0b1000_0000;
    num <<= 1;
    if msb != 0{
        num = num ^  0x1B;
    }
    return num;
}


fn galois_mul(mut num: u8, n: u8) -> u8{
    if n == 1{
        return num;
    }

    let mut new_num = num;

    if n == 2{
        new_num = double_in_galois_field(num);
        return new_num;
    }

    if n == 3{
        new_num = double_in_galois_field(num);
     new_num = new_num ^ num;
    }

    return new_num;
}



fn mix_column(state: [[u8; 4]; 4]) -> [[u8; 4]; 4]{

    let mix_matrix:[[u8; 4]; 4] = [
        [2, 3, 1, 1],
        [1, 2, 3, 1],
        [1, 1, 2, 3],
        [3, 1, 1, 2]
    ];

    let mut new_state:[[u8; 4]; 4] = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];

    for i in 0..4 {
        let col = [state[i][0], state[i][1], state[i][2], state[i][3]];
        
        new_state[i][0] = galois_mul(col[0], mix_matrix[0][0]) ^ galois_mul(col[1], mix_matrix[0][1]) ^ galois_mul(col[2], mix_matrix[0][2]) ^ galois_mul(col[3], mix_matrix[0][3]);

        new_state[i][1] = galois_mul(col[0], mix_matrix[1][0]) ^ galois_mul(col[1], mix_matrix[1][1]) ^ galois_mul(col[2], mix_matrix[1][2]) ^ galois_mul(col[3], mix_matrix[1][3]);

        new_state[i][2] = galois_mul(col[0], mix_matrix[2][0]) ^ galois_mul(col[1], mix_matrix[2][1]) ^ galois_mul(col[2], mix_matrix[2][2]) ^ galois_mul(col[3], mix_matrix[2][3]);

        new_state[i][3] = galois_mul(col[0], mix_matrix[3][0]) ^ galois_mul(col[1], mix_matrix[3][1]) ^ galois_mul(col[2], mix_matrix[3][2]) ^ galois_mul(col[3], mix_matrix[3][3]);


    }

    return new_state;
}


fn add_round_key(mut state: [[u8; 4]; 4], key: [[u8; 4]; 4]) -> [[u8; 4]; 4]{
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = state[i][j] ^ key[i][j];
        }
    }
    return state;
}

fn encrypt_block(key: [u8; 32], mut state: [[u8; 4]; 4]) -> [[u8; 4]; 4]{
    let round_keys:[[[u8; 4]; 4]; 15] = key_expansion(key);

    state = add_round_key(state, round_keys[0]);

    for i in 1..14{
        state = sub_bytes(state);
        state = shift_row(state);
        state = mix_column(state);
        state = add_round_key(state, round_keys[i]);
    }

    state = sub_bytes(state);
    state = shift_row(state);
    state = add_round_key(state, round_keys[14]);

    return state;
}


fn encrypt(key: &str, message: &str) -> String{
    let key = hex_key_to_u8_array(key).unwrap();

    let message = utf8_to_u8_array(message).unwrap();

    let ciphertext = encrypt_block(key, message);

    return array_to_hex_string(ciphertext);

}

fn array_to_hex_string(array: [[u8; 4]; 4]) -> String {
    array
        .iter()
        .flat_map(|row| row.iter())
        .map(|byte| format!("{:02x}", byte))
        .collect()
}


fn utf8_to_u8_array(input: &str) -> Result<[[u8; 4]; 4], String> {
    let bytes = input.as_bytes();

    if bytes.len() > 16 {
        return Err("The UTF-8 string contains more than 16 bytes.".to_string());
    }

    let mut array = [[0u8; 4]; 4];

    for (i, &byte) in bytes.iter().enumerate() {
        array[i / 4][i % 4] = byte;
    }

    Ok(array)
}

fn hex_key_to_u8_array(hex: &str) -> Result<[u8; 32], String> {
    if hex.len() != 64 {
        return Err("the string must be 32 characters.".to_string());
    }

    let mut array = [0u8; 32];
    for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
        let hex_str = std::str::from_utf8(chunk).map_err(|_| "Non-UTF-8 valid string.".to_string())?;
        array[i] = u8::from_str_radix(hex_str, 16).map_err(|_| format!("Unable to convert '{}' in u8.", hex_str))?;
    }

    Ok(array)
}

fn hex_to_u8_array(hex: &str) -> Result<[[u8; 4]; 4], String> {
    if hex.len() != 32 {
        return Err("the string must be 32 characters.".to_string());
    }

    let mut array = [[0u8; 4]; 4];
    for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
        let hex_str = std::str::from_utf8(chunk).map_err(|_| "Non-UTF-8 valid string.".to_string())?;
        let byte = u8::from_str_radix(hex_str, 16).map_err(|_| format!("Unable to convert '{}' in u8.", hex_str))?;
        array[i / 4][i % 4] = byte;
    }

    Ok(array)
}


fn main() {

    let key = "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4";
    let plaintext = "6bc1bee22e409f96e93d7e117393172a";

    let key = hex_key_to_u8_array(key).unwrap();
    println!("key{:?}", key);
    let plaintext = hex_to_u8_array(plaintext).unwrap();
    println!("plaintext");
    print_state_hex(plaintext);

    let ciphertext = encrypt_block(key, plaintext);

    println!("ciphertext");
    print_state_hex(ciphertext);

    println!("ciphertext: {}", array_to_hex_string(ciphertext));

}
