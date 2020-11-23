use std::iter::successors;

const ONES: [&str; 20] = [
    "nulla",
    "egy",
    "kettő",
    "három",
    "négy",
    "öt",
    "hat",
    "hét",
    "nyolc",
    "kilenc",
    "tíz",
    "tizenegy",
    "tizenkettő",
    "tizenhárom",
    "tizennégy",
    "tizenöt",
    "tizenhat",
    "tizenhét",
    "tizennyolc",
    "tizenkilenc",
];
const TENS: [(&str, &str); 10] = [
    ("nulla", "nulla"),
    ("tíz", "tizen"),
    ("húsz", "huszon"),
    ("harminc", "harminc"),
    ("negyven", "negyven"),
    ("ötven", "ötven"),
    ("hatvan", "hatvan"),
    ("hetven", "hetven"),
    ("nyolcvan", "nyolcvan"),
    ("kilencven", "kilencven"),
];
const ORDERS: [&str; 6] = [
    "nulla",
    "ezer",
    "millió",
    "milliárd",
    "billiárd",
    "trilliárd",
];

pub fn to_text(num: u64) -> String {
    encode(
        num,
        match num {
            x if x > 2000 => true,
            _ => false,
        },
    )
}

fn encode(num: u64, bttt: bool) -> String {
    match num {
        0..=19 => ONES[num as usize].to_string(),
        20..=99 => {
            let upper = (num / 10) as usize;
            match num % 10 {
                0 => TENS[upper].0.to_string(),
                lower => format!("{}{}", TENS[upper].1, encode(lower, bttt)),
            }
        }
        100..=999 => format_num(num, 100, "száz", false, bttt),
        _ => {
            let (div, order) = successors(Some(1u64), |v| v.checked_mul(1000))
                .zip(ORDERS.iter())
                .find(|&(e, _)| e > num / 1000)
                .unwrap();

            format_num(num, div, order, true, bttt)
        }
    }
}

fn format_num(num: u64, div: u64, order: &str, sep: bool, bttt: bool) -> String {
    match (num / div, num % div) {
        (upper, 0) => format!("{}{}", encode(upper, bttt), order),
        (upper, lower) => format!(
            "{}{}{}{}",
            encode(upper, bttt),
            order,
            match (sep, bttt) {
                (true, true) => "-".to_string(),
                _ => "".to_string(),
            },
            encode(lower, bttt)
        ),
    }
}
