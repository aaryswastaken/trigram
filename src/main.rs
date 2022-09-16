use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::iter::Enumerate;
use std::str;

struct Trigram {
    string: String,
    frequency: u64
}

fn main() {
    let mut source = File::open("temp.txt").unwrap();
    let mut contents = String::new();
    source.read_to_string(&mut contents).unwrap();

    let bytes = contents.as_bytes();

    let mut output: [u64; 17576] = [0; 17576];
    let mut cursor: usize = 0;

    let mut trigram_id: u64 = 0;

    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut c: u64 = 0;

    loop {
        if cursor + 2 == contents.len() {
            break;
        } else {
            c = bytes[cursor] as u64 - 97;
            b = bytes[cursor + 1] as u64 - 97;
            a = bytes[cursor + 2] as u64 - 97;

            trigram_id = a + b * 26 + c * 26 * 26;

            output[trigram_id as usize] += 1;

            cursor += 1;
        }
    }

    let mut trigrams: Vec<Trigram> = Vec::new();

    for i in 0..26 {
        for j in 0..26 {
            for k in 0..26 {
                trigrams.push(Trigram{ string: str::from_utf8(& vec![i + 97, j + 97, k + 97]).unwrap().to_string(), frequency: 0})
            }
        }
    }

    for (i, s) in output.iter().enumerate() {
        trigrams[i].frequency = *s;
    }

    trigrams.sort_by_key(|a| a.frequency);

    let mut ut = String::new();

    for t in trigrams.iter() {
        let s = format!("{},{},{}\n", t.string, t.frequency, t.frequency as f64 / (contents.len() - 2) as f64);
        println!("{}", &s);
        ut.push_str(s.as_str());
    }

    let mut dest = File::create("frequencies.txt").unwrap();

    dest.write(ut.as_bytes());
}
