trait Xor: Copy + Default {
    fn xor(&self, rhs: &Self) -> Self;
}

macro_rules! impl_xor {
    ($t:tt) => {
        impl Xor for $t {
            fn xor(&self, rhs: &Self) -> Self {
                self ^ rhs
            }
        }
    };
}

impl_xor!(u8);
impl_xor!(u16);
impl_xor!(u32);
impl_xor!(u64);
impl_xor!(usize);

// fn xor_eq<T: Xor, const N: usize>(lhs: &[T; N], rhs: &[T; N]) -> [T; N] {
//     let mut x: [T; N] = [Default::default(); N];

//     for i in 0..N {
//         x[i] = lhs[i].xor(&rhs[i]);
//     }

//     x
// }

fn xor_eq_vec<T: Xor>(lhs: &Vec<T>, rhs: &Vec<T>) -> Vec<T> {
    let mut x = Vec::new();
    let size = lhs.len().min(rhs.len());

    for i in 0..size {
        x.push(rhs[i].xor(&lhs[i]));
    }

    x
}

fn xor_1_vec<T: Xor>(lhs: &Vec<T>, xi: T) -> Vec<T> {
    let mut x = Vec::new();
    for e in lhs {
        x.push(e.xor(&xi));
    }
    x
}

fn hex_to_raw(h: char) -> u8 {
    match h {
        'a'..='f' => h as u8 - 'a' as u8 + 10,
        'A'..='F' => h as u8 - 'A' as u8 + 10,
        '0'..='9' => h as u8 - '0' as u8,
        _ => panic!("Invalid Hex string token {h}!"),
    }
}

fn raw_to_hex(raw: u8) -> [char; 2] {
    let nibble_decode = |nib: u8| match nib {
        0..=9 => (nib + '0' as u8) as char,
        10..=15 => (nib + 'a' as u8 - 10) as char,
        _ => panic!("Unreachable!"),
    };

    [nibble_decode((raw & 0xF0) >> 4), nibble_decode(raw & 0xF)]
}

fn decode_hex(hex_string: String) -> Vec<u8> {
    let chars: Vec<char> = hex_string.chars().collect();
    let pairs = (chars.len() + 1) / 2;
    let mut decoded = Vec::new();

    for i in 0..pairs {
        let upper = chars[i * 2];
        let lower = if i * 2 + 1 < chars.len() {
            chars[i * 2 + 1]
        } else {
            '0'
        };

        let coel = (hex_to_raw(upper) << 4) | hex_to_raw(lower);
        decoded.push(coel);
    }

    decoded
}

fn encode_hex(raw_bytes: Vec<u8>) -> String {
    let mut hex_string = String::new();

    for b in raw_bytes {
        let hex = raw_to_hex(b);

        hex_string.push(hex[0]);
        hex_string.push(hex[1]);
    }

    hex_string
}

fn humanness(text: &Vec<char>) -> f32 {
    const PRT: f32 = 1.0 / 100.0;

    use std::collections::HashMap;

    // frequency vector for standard english text
    let expected_freq_table = HashMap::from([
        ('A', 11.7 * PRT),
        ('B', 4.4 * PRT),
        ('C', 5.2 * PRT),
        ('D', 3.2 * PRT),
        ('E', 2.8 * PRT),
        ('F', 4.0 * PRT),
        ('G', 1.6 * PRT),
        ('H', 4.2 * PRT),
        ('I', 7.3 * PRT),
        ('J', 0.51 * PRT),
        ('K', 0.86 * PRT),
        ('L', 2.4 * PRT),
        ('M', 3.8 * PRT),
        ('N', 2.3 * PRT),
        ('O', 7.6 * PRT),
        ('P', 4.3 * PRT),
        ('Q', 0.22 * PRT),
        ('R', 2.8 * PRT),
        ('S', 6.7 * PRT),
        ('T', 16.0 * PRT),
        ('U', 1.2 * PRT),
        ('V', 0.82 * PRT),
        ('W', 5.5 * PRT),
        ('X', 0.045 * PRT),
        ('Y', 0.76 * PRT),
        ('Z', 0.045 * PRT),
    ]);

    let mut measured_freq = HashMap::new();

    for c in 'A'..='Z' {
        measured_freq.insert(c, 0.0f32);
    }

    let mut count = 0;
    let alphabet: Vec<char> = ('A'..='Z').collect();
    for c in text {
        if alphabet.contains(&c.to_ascii_uppercase()) {
            count = count + 1;
        }
    }

    //println!("Found {count} plain characters.");
    if count == 0 {
        return 0.0;
    }
    //let plain_char_ratio = count as f32 / text.len() as f32;

    // measure letter frequency
    let finc = 1.0 / count as f32;
    for c in text {
        let c_upper = c.to_ascii_uppercase();
        if alphabet.contains(&c_upper) {
            measured_freq
                .entry(c_upper)
                .and_modify(|value| *value = *value + finc)
                .or_insert(0.0);
        }
    }

    let mut freq_drift = 0.0;

    for c in &alphabet {
        let expected = *expected_freq_table.get(c).unwrap_or(&0.0);
        let measured = *measured_freq.get(c).unwrap_or(&0.0);

        freq_drift += (expected - measured).abs() / 24.0;
    }

    let weird_symbols = "&%$\\/#@$^*()+_<>|{}";
    let mut weird_count = 0.0;

    for c in text {
        if weird_symbols.find(*c).is_some() {
            weird_count += 1.0;
        }
    }
    let weird_factor = weird_count / (text.len() as f32);

    let mut invalid_count = 0.0;
    for c in text {
        if *c > 127 as char {
            invalid_count += 1.0;
        }
    }
    let invalid_factor = invalid_count / (text.len() as f32);

    // compute score
    100.0 / freq_drift + 1_000.0 * (1.0 - weird_factor) + 50_000.0 * (1.0 - invalid_factor)
}

fn to_ascii(raw: Vec<u8>) -> Vec<char> {
    let mut ascii = Vec::new();

    for r in raw {
        ascii.push(r.into());
    }

    ascii
}

fn xor1x_and_score(encrypted: &Vec<u8>, xor_cipher: u8) -> (f32, Vec<char>) {
    let new = xor_1_vec(encrypted, xor_cipher);
    let ascii = to_ascii(new);
    (humanness(&ascii), ascii)
}

fn array_to_string(value: Vec<char>) -> String {
    let mut string = String::new();

    for c in value {
        string.push(c);
    }

    string
}

#[derive(Debug, Clone)]
struct ScoredResult(pub f32, pub u8, pub String);

fn get_top(hex: String, n: usize) -> Vec<ScoredResult> {
    let dcs = decode_hex(hex);
    let mut results = Vec::new();

    for cipher in 0..=u8::MAX {
        let (score, ascii) = xor1x_and_score(&dcs, cipher);
        //println!("{:?}\t|{:?}\t\t[{:?}]", score, cipher, array_to_string(ascii));
        results.push((score, cipher, array_to_string(ascii)));
    }

    results.sort_by(|x, y| {
        if x.0 > y.0 {
            std::cmp::Ordering::Greater
        } else if x.0 == y.0 {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Less
        }
    });
    results.reverse();

    let mut topn = Vec::new();
    for i in 0..n {
        topn.push(ScoredResult(
            results[i].0,
            results[i].1,
            results[i].2.clone(),
        ));
    }

    topn
}

#[derive(Debug)]
struct Settings {
    help: bool,
    input_is_hex: bool,
    hex_string: Option<String>,
    eq_xor: bool,
    eq_hex_comp: Option<String>,
    find_1xor: bool,
    find_1xor_candidates: bool,
    file_path: Option<String>,
}

fn main() {
    let settings = get_settings();

    if settings.help {
        display_settings(&settings);
    }

    let mut input = String::new();

    if settings.input_is_hex {
        input = settings.hex_string.unwrap();
    }

    if settings.eq_xor {
        let cmp = settings.eq_hex_comp.unwrap();

        let lhs = decode_hex(input);
        let rhs = decode_hex(cmp);

        let xor_result = xor_eq_vec(&lhs, &rhs);
        println!("{:}", encode_hex(xor_result));
    } else if settings.find_1xor {
        let dcs = decode_hex(input);
        let mut results = Vec::new();

        for cipher in 0..=u8::MAX {
            let (score, ascii) = xor1x_and_score(&dcs, cipher);
            //println!("{:?}\t|{:?}\t\t[{:?}]", score, cipher, array_to_string(ascii));
            results.push((score, cipher, array_to_string(ascii)));
        }

        results.sort_by(|x, y| {
            if x.0 > y.0 {
                std::cmp::Ordering::Greater
            } else if x.0 == y.0 {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Less
            }
        });
        results.reverse();

        for (score, cipher, message) in results {
            println!("{:>10?}|{:>3?}>[{:?}]", score, cipher, message);
        }
    } else if settings.find_1xor_candidates {
        use std::collections::HashMap;

        let input = std::fs::read_to_string(settings.file_path.unwrap()).unwrap();
        let candidates = input.split("\n");
        let mut best_guesses = HashMap::new();
        let cut_off = 52_000.0f32;

        for line in candidates {
            let line_string = String::from(line);
            let top = get_top(line_string, 5);
            best_guesses.insert(line, top);
        }

        for (k, v) in best_guesses.into_iter() {
            let mut max = v[0].0;
            for e in &v {
                if e.0 > max {
                    max = e.0;
                }
            }
            if max > cut_off {
                println!("[{k:?}] : {v:?}");
            }
        }
    }
}

fn display_settings(settings: &Settings) {
    println!("{:?}", settings);
}

fn get_settings() -> Settings {
    Settings {
        help: find_switch("-help").is_some()
            || find_switch("-?").is_some()
            || find_switch("-v").is_some(),
        input_is_hex: find_switch("-h").is_some(),
        hex_string: get_switch_field("-h"),
        eq_xor: find_switch("-exor").is_some(),
        eq_hex_comp: get_switch_field("-exor"),
        find_1xor: find_switch("-f1xor").is_some(),
        find_1xor_candidates: find_switch("-f1xc").is_some(),
        file_path: get_switch_field("-f1xc"),
    }
}

fn find_switch(switch: &str) -> Option<String> {
    std::env::args().find(|element| element.contains(switch))
}

fn get_switch_field(switch: &str) -> Option<String> {
    let mut full = find_switch(switch)?;

    // format is -[azAZ09]=<field>
    let index = full.find("=")?;

    Some(full.split_off(index + 1))
}
