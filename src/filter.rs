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
    ("b", ":b:"),
    ("e", "3"),
    ("i", "1"),
    ("m", ":m:"),
    ("o", "0"),
    ("s", "5"),
    ("t", "7"),
    ("v", ":b:"),
    ("l", "]"),
];

impl Filter {
    pub fn filtered(&self, s: &mut String) {
        *s = self.filter(s);
    }

    pub fn filter(&self, s: &str) -> String {
        match self {
            Self::Benti => {
                let mut r = String::new();
                let s = s.to_lowercase().replace("i dont know", "me no no");

                for (i, w) in s.split_whitespace().enumerate() {
                    let b_chance = rand_limit(1, (i+1).min(4) as u64);

                    let mut w = w.replace("ate", "8").replace("ait", "8");

                    if let Some(c) = w.chars().nth(0) {
                        if c == 'p' {
                            w = "b".to_string() + &w[1..];
                        }
                    }

                    w = replace_multi(BENTI_WORD_RT, &w);

                    if b_chance == 1 {
                        w = "b".to_string() + &w[1..];
                    }

                    r += &(replace_multi(BENTI_CHAR_RT, &w) + " ");
                }

                let xd = "xd".repeat(rand_limit(0, 11) as usize % 4);

                r + &xd
            },
        }
    }
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

fn srand(seed: u64) {
    unsafe {
        RAND_SEED = seed;
    }
}

fn rand_limit(l: u64, u: u64) -> u64 {
    (rand() % (u - l + 1)) + l
}
