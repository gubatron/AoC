use md5::digest::core_api::CoreWrapper;
use md5::{Digest, Md5, Md5Core};

fn concat_secret_to_i32(secret: &str, n: i32) -> String {
    format!("{}{}", secret, n)
}

fn hash_has_5_zeroes(hash_str: &String) -> bool {
    hash_str.starts_with("00000")
}

fn md5hash_string(input: &String) -> String {
    let mut hasher: CoreWrapper<Md5Core> = Md5::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn find_adventcoin_nonce(secret: &String) -> i32 {
    let mut i = 0;
    loop {
        let nonce = concat_secret_to_i32(secret, i);
        let md5_hash_string = &md5hash_string(&nonce);
        if hash_has_5_zeroes(md5_hash_string) {
            return i;
        }
        i += 1;
    }
}

#[test]
fn tests() {
    assert_eq!(find_adventcoin_nonce(&"abcdef".to_string()), 609043);
    assert_eq!(find_adventcoin_nonce(&"pqrstuv".to_string()), 1048970);
}

fn part1() {
    println!("{}", find_adventcoin_nonce(&"bgvyzdsv".to_string()));
}

fn main() {
    part1();
}
