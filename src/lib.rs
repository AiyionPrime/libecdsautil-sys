#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use hex;
    // secret: c022af10206c23f7e1f96a538bf9df814eeea39a9300c594b3473593e19c5d66
    // public: aa353cd1b3e107ff2b6c1957fcabd8656928a07858732b56227a9f6536de7197

    #[test]

    fn test_ecdsa_sign_legacy() {
        // hash is the one of an empty file
        unsafe {
            let expected_signature = "112e4763181666264fce3d6c670070d16e5484711958164e5b89b3772e69c304330db21764b2ad36b498806862c398b2304cb73ebb6c1f382d94f2d4ffbc0c02";
            let expected_r = hex::decode(&expected_signature[..64]).expect("Decoding failed");
            let expected_s = hex::decode(&expected_signature[64..]).expect("Decoding failed");

            let hash_bytes = hex::decode("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855").expect("Decoding failed");
            let secret_bytes = hex::decode("c022af10206c23f7e1f96a538bf9df814eeea39a9300c594b3473593e19c5d66").expect("Decoding failed");

            let mut signature: ecdsa_signature_t = mem::zeroed();
            let hash: *const ecc_int256_t = hash_bytes.as_ptr() as *const ecc_int256_t;
            let secret: *const ecc_int256_t = secret_bytes.as_ptr() as *const ecc_int256_t;

            ecdsa_sign_legacy(&mut signature as *mut _, hash, secret);

            let r = signature.r.p;
            let s = signature.s.p;

            assert_eq!(expected_r, r);
            assert_eq!(expected_s, s);
        }
    }
}
