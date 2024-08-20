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

fn xor_eq<T: Xor, const N: usize>(lhs: &[T; N], rhs: &[T; N]) -> [T; N] {
    let mut x: [T; N] = [Default::default(); N];

    for i in 0..N {
        x[i] = lhs[i].xor(&rhs[i]);
    }

    x
}

fn xor_eq_vec<T: Xor>(lhs: &Vec<T>, rhs: &Vec<T>) -> Vec<T> {
    let mut x = Vec::new();
    let size = lhs.len().min(rhs.len());

    for i in 0..size {
        x.push(rhs[i].xor(&lhs[i]));
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

#[derive(Debug)]
struct Settings {
    help: bool,
    input_is_hex: bool,
    hex_string: Option<String>,
    eq_xor: bool,
    eq_hex_comp: Option<String>,
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
