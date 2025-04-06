use aes::Aes128;
use cipher::generic_array::GenericArray;
use cipher::{BlockEncrypt, KeyInit};

/// XOR two 16-byte arrays
fn xor_128(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = a[i] ^ b[i];
    }
    out
}

/// Security function e
/// 
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-c28faf63-e654-5969-3786-00143319cae7
pub (self) fn e(
    key: &[u8; 16], // 128-bit key
    plaintext: &[u8; 16], // 128-bit plaintext
) -> [u8; 16] {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut block = GenericArray::clone_from_slice(plaintext);
    cipher.encrypt_block(&mut block);
    block.into()
}

/// Random address hash function ah
/// 
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-03b4d5c9-160c-658a-7aa5-d0b2230d38f1
fn ah(
    k: &[u8; 16], // 128-bit key
    r: &[u8; 3], // 24-bit plaintext
) -> [u8; 16] {
    let r_prime: [u8; 16] = [r[0], r[1], r[2], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    e(k, &r_prime)
}

/// Key generation function s1 for LE legacy pairing
/// 
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-df36abdc-4d80-8f1b-0ef7-fbcae4d25825
fn s1(
    k: &[u8; 16], // 128-bit key
    r1: &[u8; 16], // 128-bit plaintext
    r2: &[u8; 16], // 128-bit plaintext
) -> [u8; 16] {
    e(k, &[
        r1[0], r1[1], r1[2], r1[3], r1[4], r1[5], r1[6], r1[7],
        r2[0], r2[1], r2[2], r2[3], r2[4], r2[5], r2[6], r2[7],
    ])
}



/// Confirm value generation function c1 for LE legacy pairing
/// 
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-24e06a05-2f0b-e5c9-7c65-25827ddb9975
/// 
/// k is 128 bits
/// r is 128 bits
/// pres is 56 bits
/// preq is 56 bits
/// iat is 1 bit
/// ia is 48 bits
/// rat is 1 bit
/// ra is 48 bits
/// padding is 32 zero bits
pub fn c1(
    k: &[u8; 16], // 128-bit key
    r: &[u8; 16], // 128-bit plaintext
    pres: &[u8; 7], // 56-bit plaintext
    preq: &[u8; 7], // 56-bit plaintext
    iat: u8, // 1 bit
    ia: &[u8; 6], // 48-bit plaintext
    rat: u8, // 1 bit
    ra: &[u8; 6], // 48-bit plaintext
) -> [u8; 16] {

    let p1 = [
        iat & 0x01, rat & 0x01,
        preq[0], preq[1], preq[2], preq[3], preq[4], preq[5], preq[6],
        pres[0], pres[1], pres[2], pres[3], pres[4], pres[5], pres[6],
    ];
    let p2 = [
        ra[0], ra[1], ra[2], ra[3], ra[4], ra[5],
        ia[0], ia[1], ia[2], ia[3], ia[4], ia[5],
        0, 0, 0, 0
    ];
    e(k, &xor_128(&e(k, &xor_128(r, &p1)), &p2))
}

// --- Test Module ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e() {
        // FIPS 197, Advanced Encryption Standard (AES)
        //
        // C.1 AES-128 (Nk=4, Nr=10)
        // https://csrc.nist.gov/files/pubs/fips/197/final/docs/fips-197.pdf
        // 
        // AES-128 ECB
        // plaintext = 00112233445566778899aabbccddeeff
        // key = 000102030405060708090a0b0c0d0e0f
        // output = 69c4e0d86a7b0430d8cdb78070b4c55a
        let plaintext: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        ];
        let key: [u8; 16] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        ];
        let expected: [u8; 16] = [
            0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30,
            0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4, 0xc5, 0x5a,
        ];
        let result = e(&key, &plaintext);
        assert_eq!(result, expected);
    }


    // use super::*; // Import functions from outer scope

    // const sent_le_set_random_address: [u8; 10] = [0x1, 0x5, 0x20, 0x6, 0x6, 0x33, 0x74, 0xd6, 0x56, 0xd3];

    // const rcvd_le_connection_complete: [u8; 22] = [
    //     0x4, 0x3e, 0x13, 0x1, 0x0, 0x40, 0x0, 0x1, 0x0, 0x26, 0xe, 0xd6, 0xe8, 0xc2, 0x50, 0x30,
    //     0x0, 0x0, 0x0, 0xc0, 0x3, 0x1,
    // ];

    // const rcvd_pairing_request: [u8; 16] = [
    //     0x2, 0x40, 0x20, 0xb, 0x0, 0x7, 0x0, 0x6, 0x0, 0x1, 0x4, 0x0, 0x2d, 0x10, 0xe, 0xf,
    // ];

    // const sent_pairing_response: [u8; 16] = [
    //     0x2, 0x40, 0x0, 0xb, 0x0, 0x7, 0x0, 0x6, 0x0, 0x2, 0x3, 0x0, 0x1, 0x10, 0x0, 0x1,
    // ];

    // const rcvd_pairing_confirm: [u8; 26] = [
    //     0x2, 0x40, 0x20, 0x15, 0x0, 0x11, 0x0, 0x6, 0x0, 0x3, 0x4f, 0x40, 0x76, 0x6b, 0x82, 0xfc,
    //     0xa8, 0xa6, 0x5, 0x14, 0x99, 0x54, 0xf5, 0xa4, 0x4b, 0x3a,
    // ];

    // const sent_pairing_confirm: [u8; 26] = [
    //     0x2, 0x40, 0x0, 0x15, 0x0, 0x11, 0x0, 0x6, 0x0, 0x3, 0xdc, 0x79, 0x69, 0x34, 0xd6, 0x98,
    //     0xba, 0xd9, 0xfd, 0x91, 0x79, 0x1f, 0x58, 0x25, 0x8, 0x4f,
    // ];

    // const rcvd_pairing_random: [u8; 26] = [
    //     0x2, 0x40, 0x20, 0x15, 0x0, 0x11, 0x0, 0x6, 0x0, 0x4, 0x65, 0x5, 0x8d, 0x11, 0xb, 0x62,
    //     0x4b, 0x30, 0xdd, 0x7f, 0x7d, 0x73, 0x20, 0xf2, 0xec, 0x74,
    // ];

    // const sent_pairing_random: [u8; 26] = [
    //     0x2, 0x40, 0x0, 0x15, 0x0, 0x11, 0x0, 0x6, 0x0, 0x4, 0x6d, 0xde, 0x61, 0xf5, 0x68, 0x16,
    //     0x96, 0x67, 0x8a, 0x5e, 0x28, 0x70, 0x1a, 0x34, 0x38, 0x0,
    // ];


}
