use std::cell::RefCell;
use std::io::{self, Read};

thread_local! {
    // Cの static FILE *szpFpI 相当（ここではstdin全体を文字列として保持し、行単位で読む）
    static INPUT: RefCell<Input> = RefCell::new(Input::new(String::new()));
    // Cの static double sdRate 相当
    static SD_RATE: RefCell<f64> = RefCell::new(0.0);
}

struct Input {
    data: String,
    pos: usize,
}
impl Input {
    fn new(data: String) -> Self {
        Self { data, pos: 0 }
    }

    // Cの fgets(buf, sizeof(buf), fp) 相当（最大1023文字＋終端を想定）
    // - 改行があれば含めて返す
    // - EOFなら None
    // - 行が1023文字を超える場合はそこで切る（Cのfgetsと同様に「行の残り」は次回に回る）
    fn fgets_1024(&mut self) -> Option<String> {
        if self.pos >= self.data.len() {
            return None;
        }
        let bytes = self.data.as_bytes();
        let start = self.pos;

        // 最大読み取り長: 1023
        let mut end = start;
        let max_end = (start + 1023).min(bytes.len());
        while end < max_end {
            end += 1;
            if bytes[end - 1] == b'\n' {
                break;
            }
        }
        self.pos = end;
        Some(self.data[start..end].to_string())
    }
}

// Cの fOutLine: D_TESTなしでは printf("%s", line)
fn f_out_line(line: &str) -> i32 {
    print!("{line}");
    0
}

// Cの fGetVal をそのまま再帰で移植（メモ化等はしない：Cコードの挙動に合わせる）
fn f_get_val(pos: i32) -> f64 {
    if pos % 2 == 0 {
        // (double)(piPos / 2) / sdRate;
        // Cの整数除算の丸め（0方向）をRustでも再現（i32の / は0方向）
        let half = pos / 2;
        let rate = SD_RATE.with(|r| *r.borrow());
        (half as f64) / rate
    } else {
        let rate = SD_RATE.with(|r| *r.borrow());
        let mut val = 1.0f64;
        val += f_get_val(pos - 1) * rate;
        val += f_get_val(pos + 1) * (1.0 - rate);
        val
    }
}

fn f_main() -> i32 {
    // 座標 - 取得
    let line1 = INPUT.with(|inp| inp.borrow_mut().fgets_1024()).unwrap_or_default();
    let li_pos: i32 = line1.split_whitespace().next().unwrap_or("0").parse().unwrap_or(0);

    // 近づく確率 - 取得
    let line2 = INPUT.with(|inp| inp.borrow_mut().fgets_1024()).unwrap_or_default();
    let mut rate: f64 = line2
        .split_whitespace()
        .next()
        .unwrap_or("0")
        .parse()
        .unwrap_or(0.0);
    rate /= 100.0;
    SD_RATE.with(|r| *r.borrow_mut() = rate);

    // 期待値 - 取得
    let val = f_get_val(li_pos);

    // 出力: sprintf(lc1Buf, "%.7lf\n", ldVal);
    // Rustの表示はCのprintf("%.7lf")に近い丸めを行う
    let out = format!("{:.7}\n", val);
    f_out_line(&out);

    0
}

fn f_one() -> i32 {
    // Cの非D_TESTでは szpFpI = stdin; なので、stdin全体を読み込んでfgets相当で処理する
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    INPUT.with(|inp| *inp.borrow_mut() = Input::new(s));

    let _ = f_main();
    0
}

fn main() {
    let _ = f_one();
}
