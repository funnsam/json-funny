use clap::*;
use filter::*;
use serde_json::*;

mod filter;

#[derive(Debug, Clone, Parser)]
struct Args {
    file: String,
    #[clap(value_enum)]
    filter: Filter,

    output: String,
}

fn main() {
    srand(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());

    let args = Args::parse();

    let file = std::fs::read(args.file).unwrap();
    let mut file = from_slice(&file).unwrap();

    filter(&mut file, &args.filter);

    std::fs::write(args.output, to_vec(&file).unwrap()).unwrap();
}

fn filter(val: &mut Value, fil: &Filter) {
    match val {
        Value::String(ref mut s) => fil.filtered(s),
        Value::Array(ref mut a) => for v in a.iter_mut() {
            filter(v, fil);
        },
        Value::Object(ref mut f) => for v in f.iter_mut() {
            filter(v.1, fil);
        }
        _ => {},
    }
}
