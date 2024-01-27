use clap::*;

#[derive(Debug, Clone, ValueEnum)]
pub enum Filter {
    Benti,
}

type ReplaceTable = &'static [(&'static str, &'static str)];

const BENTI_WORD_RT: ReplaceTable = &[
    ("when", "wen"),
    ("then", "den"),
    ("cant", "kant"),
    ("i", "me"),
    ("know", "no"),
    ("be", "b"),
    ("color", "kolor"),
    ("colour", "kolor"),
    ("there", "dere"),
    ("to", "2"),
    ("block", "blocc"),
    ("oh", "o"),
    ("can", "kan"),
    ("milk", "miwk"),
    ("thats", "dats"),
    ("just", "jus"),
    ("because", "cuz"),
    ("thanks", "thx"),
    ("though", "thugh"),
];

const BENTI_CHAR_RT: ReplaceTable = &[
    ("a", "4"),
    ("b", "\u{1F171}"),
    ("e", "3"),
    ("i", "1"),
    ("m", "\u{24C2}"),
    ("o", "0"),
    ("s", "5"),
    ("t", "7"),
    ("v", "\u{1F171}"),
    ("l", "]"),
    ("p", "["),
];

impl Filter {
    pub fn filtered(&self, s: &mut String) {
        *s = self.filter(s);
    }

    pub fn filter(&self, s: &str) -> String {
        match self {
            Self::Benti => {
                let mut r = String::new();
                let s = s.to_lowercase().replace("dont know", "no no");

                for (i, w) in s.split_whitespace().enumerate() {
                    if is_formatting(w) {
                        r += w;
                        r += " ";
                        continue;
                    }

                    let b_chance = rand_limit(1, (i+1).min(4) as u64);

                    let mut w = w.replace("ate", "8").replace("ait", "8");

                    if let Some(c) = w.chars().nth(0) {
                        if c == 'p' {
                            w = "b".to_string() + &w.chars().skip(1).collect::<String>();
                        }
                    }

                    w = replace_all(BENTI_WORD_RT, &w);

                    if b_chance == 1 {
                        w = "b".to_string() + &w.chars().skip(1).collect::<String>();
                    }

                    r += &(replace_multi(BENTI_CHAR_RT, &w) + " ");
                }

                let xd = "xd".repeat(rand_limit(0, 11) as usize / 4);

                r.chars().take(r.chars().count()-1).collect::<String>() + &xd
            },
        }
    }
}

fn replace_all(rt: ReplaceTable, s: &str) -> String {
    for (o, n) in rt.iter() {
        if &s == o {
            return n.to_string();
        }
    }

    s.to_string()
}

fn replace_multi(rt: ReplaceTable, s: &str) -> String {
    let mut s = s.to_string();

    for (o, n) in rt.iter() {
        s = s.replace(o, n);
    }

    s
}

static mut RAND_SEED: u64 = 1;

fn rand() -> u64 {
    unsafe {
        let mut x = RAND_SEED;
        // if x == 0 {x = now() as u64;}
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        RAND_SEED = x;
        x
    }
}

pub fn srand(seed: u64) {
    unsafe {
        RAND_SEED = seed;
    }
}

fn rand_limit(l: u64, u: u64) -> u64 {
    (rand() % (u - l + 1)) + l
}

fn is_formatting(s: &str) -> bool {
    let mut i = s.chars();
    for j in i.by_ref() {
        if j == '%' {
            return if i.count() != 0 {
                true
            } else {
                false
            }
        }
    }

    false
}
