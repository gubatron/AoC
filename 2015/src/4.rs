use md5::digest::core_api::CoreWrapper;
use md5::{Digest, Md5, Md5Core};

fn md5hash_string(input: &String) -> String {
    let mut hasher: CoreWrapper<Md5Core> = Md5::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn find_adventcoin_nonce(secret: &String, has_six_zeroes: bool) -> i32 {
    let mut nonce = 0;
    loop {
        let hash_input = format!("{secret}{nonce}");
        let md5_hash_string = &md5hash_string(&hash_input);
        if !has_six_zeroes && md5_hash_string.starts_with("00000") {
            return nonce;
        } else if has_six_zeroes && md5_hash_string.starts_with("000000") {
            return nonce;
        }
        nonce += 1;
    }
}

#[test]
fn tests() {
    assert_eq!(find_adventcoin_nonce(&"abcdef".to_string(), false), 609043);
    assert_eq!(
        find_adventcoin_nonce(&"pqrstuv".to_string(), false),
        1048970
    );
}

fn part1() {
    println!("{}", find_adventcoin_nonce(&"bgvyzdsv".to_string(), false));
}
fn part2() {
    println!("{}", find_adventcoin_nonce(&"bgvyzdsv".to_string(), true));
}

fn main() {
    part1(); // 254575
    part2(); // 1038736
}
