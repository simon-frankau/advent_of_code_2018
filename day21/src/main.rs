use std::collections::HashSet;

fn main() {
    let mut seen = HashSet::new();

    let mut r4: i64 = 0;
    let mut r1: i64 = 0;
    loop {
        r1 = r4 | 0x010000;
        r4 = 0xF49DE8;

        while r1 > 0 {
            r4 += r1 & 0xff;
            r4 &= 0xffffff;
            r4 *= 0x01016B;
            r4 &= 0xffffff;
            r1 = r1 >> 8;
        }

        if seen.contains(&r4) {
            break;
        }
        println!("{}\n", r4);
        seen.insert(r4);
    }
}
