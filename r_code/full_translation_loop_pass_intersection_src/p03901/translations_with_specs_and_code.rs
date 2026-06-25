use std::io::{self, Read};

fn get_val(pos: i32, rate: f64) -> f64 {
    // Even position: (pos/2) / rate, with integer division on pos/2 as in C.
    if pos % 2 == 0 {
        return (pos / 2) as f64 / rate;
    }

    // Odd position: 1 + val(pos-1)*rate + val(pos+1)*(1-rate)
    1.0 + get_val(pos - 1, rate) * rate + get_val(pos + 1, rate) * (1.0 - rate)
}

fn main() {
    // Read entire stdin; spec assumes at least two readable lines with valid values.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let pos: i32 = it.next().unwrap().parse().unwrap();
    let mut rate: f64 = it.next().unwrap().parse().unwrap();
    rate /= 100.0;

    let val = get_val(pos, rate);
    print!("{:.7}\n", val);
}
