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

    #[clap(short, long)]
    skip: Option<Vec<String>>,
}

fn main() {
    srand(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());

    let args = Args::parse();

    let file = std::fs::read(args.file).unwrap();
    let mut file = from_slice(&file).unwrap();

    let skip = args.skip.unwrap_or(Vec::new());

    filter(&mut file, &args.filter, &skip);

    std::fs::write(args.output, to_vec(&file).unwrap()).unwrap();
}

fn filter(val: &mut Value, fil: &Filter, skip: &Vec<String>) {
    match val {
        Value::String(ref mut s) => fil.filtered(s),
        Value::Array(ref mut a) => for v in a.iter_mut() {
            filter(v, fil, skip);
        },
        Value::Object(ref mut f) => for v in f.iter_mut() {
            if !skip.contains(v.0) {
                filter(v.1, fil, skip);
            }
        }
        _ => {},
    }
}
