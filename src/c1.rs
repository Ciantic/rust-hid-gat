use aes::Aes128;
use cipher::generic_array::GenericArray;
use cipher::{BlockEncrypt, KeyInit};

#[cfg(test)]
fn dump_bytes_as_hex(bytes: &[u8], len: usize) {
    for byte in bytes.iter().take(len) {
        print!("{:02x} ", byte);
    }
    println!();
}

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
pub fn s1(
    k: &[u8; 16], // 128-bit key
    r1: &[u8; 16], // 128-bit plaintext
    r2: &[u8; 16], // 128-bit plaintext
) -> [u8; 16] {
    e(k, &[
        r1[8], r1[9], r1[10], r1[11], r1[12], r1[13], r1[14], r1[15],
        r2[8], r2[9], r2[10], r2[11], r2[12], r2[13], r2[14], r2[15],
    ])
}



/// Confirm value generation function c1 for LE legacy pairing
/// 
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-24e06a05-2f0b-e5c9-7c65-25827ddb9975
/// 
/// k is 128 bits, is full zeroes in Just Works
/// r is 128 bits, random value
/// pres is 56 bits, 
/// preq is 56 bits
/// iat is 1 bit
/// ia is 48 bits
/// rat is 1 bit
/// ra is 48 bits
/// padding is 32 zero bits
pub fn c1(
    k: &[u8; 16], 
    r: &[u8; 16], 
    pres: &[u8; 7], 
    preq: &[u8; 7], 
    iat: u8, 
    ia: &[u8; 6], 
    rat: u8, 
    ra: &[u8; 6], 
) -> [u8; 16] {

    let p1 = [
        pres[0], pres[1], pres[2], pres[3], pres[4], pres[5], pres[6],
        preq[0], preq[1], preq[2], preq[3], preq[4], preq[5], preq[6],
        rat & 0x01, iat & 0x01
    ];
    #[cfg(test)]
    {
        print!("[p1] ");
        dump_bytes_as_hex(&p1, 16);
    }
    let p2 = [
        0, 0, 0, 0,
        ia[0], ia[1], ia[2], ia[3], ia[4], ia[5],
        ra[0], ra[1], ra[2], ra[3], ra[4], ra[5],
    ];
    
    #[cfg(test)]
    {
        print!("[p2] ");
        dump_bytes_as_hex(&p2, 16);
    }
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

    #[test]
    fn test_c1() {
        // Example values from: 
        // https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-24e06a05-2f0b-e5c9-7c65-25827ddb9975
        //
        // For example, if the 128-bit k is 0x00000000000000000000000000000000, the
        // 128-bit value r is 0x5783D52156AD6F0E6388274EC6702EE0, the 128-bit value
        // p1 is 0x05000800000302070710000001010001 and the 128-bit value p2 is
        // 0x00000000A1A2A3A4A5A6B1B2B3B4B5B6 then the 128-bit output from the c1
        // function is 0x1E1E3FEF878988EAD2A74DC5BEF13B86.
        // 
        // For example, if the 8-bit iat’ is 0x01, the 8-bit rat’ is 0x00, the
        // 56-bit preq is 0x07071000000101 and the 56 bit pres is 0x05000800000302
        // then p1 is 0x05000800000302070710000001010001.
        //
        // For example, if 48-bit ia is 0xA1A2A3A4A5A6 and the 48-bit ra is
        // 0xB1B2B3B4B5B6 then p2 is 0x00000000A1A2A3A4A5A6B1B2B3B4B5B6.

        let k: [u8; 16] = [0x00; 16];
        let r: [u8; 16] = [
            0x57, 0x83, 0xd5, 0x21, 0x56, 0xad, 0x6f, 0x0e,
            0x63, 0x88, 0x27, 0x4e, 0xc6, 0x70, 0x2e, 0xe0,
        ];
        let pres : [u8; 7] = [
            0x05, 0x00, 0x08, 0x00, 0x00, 0x03, 0x02,
        ];
        let preq : [u8; 7] = [
            0x07, 0x07, 0x10, 0x00, 0x00, 0x01, 0x01,
        ];
        let iat: u8 = 0x01;
        let rat: u8 = 0x00;
        let ia: [u8; 6] = [
            0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6,
        ];
        let ra: [u8; 6] = [
            0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6,
        ];
        let result = c1(&k, &r, &pres, &preq, iat, &ia, rat, &ra);

        
        let expected: [u8; 16] = [
            0x1e, 0x1e, 0x3f, 0xef, 0x87, 0x89, 0x88, 0xea,
            0xd2, 0xa7, 0x4d, 0xc5, 0xbe, 0xf1, 0x3b, 0x86,
        ];
        assert_eq!(result, expected);
    }


    #[test]
    fn test_s1() {
        // Example values from: 
        // https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-df36abdc-4d80-8f1b-0ef7-fbcae4d25825
        //
        // r1 is 0x000F0E0D0C0B0A091122334455667788
        // r2 is 0x010203040506070899AABBCCDDEEFF00
        // k is 0x00000000000000000000000000000000
        // output is 0x9a1fe1f0e8b0f49b5b4216ae796da062
        let k: [u8; 16] = [0x00; 16];
        let r1: [u8; 16] = [
            0x00, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09,
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
        ];
        let r2: [u8; 16] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00,
        ];
        let expected: [u8; 16] = [
            0x9a, 0x1f, 0xe1, 0xf0, 0xe8, 0xb0, 0xf4, 0x9b,
            0x5b, 0x42, 0x16, 0xae, 0x79, 0x6d, 0xa0, 0x62,
        ];
        let result = s1(&k, &r1, &r2);
        assert_eq!(result, expected);

    }

    #[test]
    fn test_c1_2() {
        // This example is from packet capture, and shows how the values needs
        // to be reversed.

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

        let mut expected_rcvd_confirm_value = [0x4f, 0x40, 0x76, 0x6b, 0x82, 0xfc, 0xa8, 0xa6, 0x5, 0x14, 0x99, 0x54, 0xf5, 0xa4, 0x4b, 0x3a];
        let mut expected_sent_confirm_value = [0xdc, 0x79, 0x69, 0x34, 0xd6, 0x98, 0xba, 0xd9, 0xfd, 0x91, 0x79, 0x1f, 0x58, 0x25, 0x8, 0x4f];

        let client_address = [0x26u8, 0xe, 0xd6, 0xe8, 0xc2, 0x50];
        let server_address = [0x6u8, 0x33, 0x74, 0xd6, 0x56, 0xd3];

        let k = [0x00u8; 16];
        let mut client_random = [0x65u8, 0x5, 0x8d, 0x11, 0xb, 0x62, 0x4b, 0x30, 0xdd, 0x7f, 0x7d, 0x73, 0x20, 0xf2, 0xec, 0x74];
        let mut server_random = [0x6du8, 0xde, 0x61, 0xf5, 0x68, 0x16, 0x96, 0x67, 0x8a, 0x5e, 0x28, 0x70, 0x1a, 0x34, 0x38, 0x0];
        let mut pres = [0x2u8, 0x3, 0x0, 0x1, 0x10, 0x0, 0x1];
        let mut preq = [0x1u8, 0x4, 0x0, 0x2d, 0x10, 0xe, 0xf];
        let iat = 0x0u8; 
        let mut ia = client_address;
        let rat = 0x1u8; // Random
        let mut ra = server_address;

        // Reverse the values to match the order in the spec
        pres.reverse();
        preq.reverse();
        ia.reverse();
        ra.reverse();
        client_random.reverse();
        server_random.reverse();

        // Validate confirm value from receiver
        let result = c1(&k, &client_random, &pres, &preq, iat, &ia, rat, &ra);
        expected_rcvd_confirm_value.reverse();
        assert_eq!(result, expected_rcvd_confirm_value);

        // Validate confirm value from sender
        let result = c1(&k, &server_random, &pres, &preq, iat, &ia, rat, &ra);
        expected_sent_confirm_value.reverse();
        assert_eq!(result, expected_sent_confirm_value);
    }
}
