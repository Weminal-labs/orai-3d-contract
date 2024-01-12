use std::time::{SystemTime, UNIX_EPOCH};

fn xorshift(mut seed: u64) -> u64 {
    seed ^= seed << 13;
    seed ^= seed >> 7;
    seed ^= seed << 17;
    seed
}

pub fn generate_random_string(length: usize) -> String {
    static mut COUNTER: u64 = 0;

    let start_time = SystemTime::now();
    let since_epoch = start_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let seed = since_epoch.as_secs() + unsafe { COUNTER };
    unsafe { COUNTER += 1 };

    let mut rng = xorshift(seed);

    let alphabet: Vec<char> = (b'a'..=b'z').map(char::from).collect();

    let random_string: String = (0..length)
        .map(|_| {
            let index = (rng % alphabet.len() as u64) as usize;
            rng = xorshift(rng);
            alphabet[index]
        })
        .collect();

    random_string
}