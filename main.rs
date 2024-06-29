use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut stdout = BufWriter::new(stdout());
    let mut acc = 11719107999768421026u128;

    for i in 0u32.. {
        for _ in 0..94u32.pow(i) {
            let _ = stdout.write_all(&(acc - 9404222468949967490).to_le_bytes()[..i as usize]);
            let _ = stdout.write_all(&[b'\n']);
            acc += 1 + ((!acc & 9259542123273814144) >> 7) * 162;
        }
    }
}
