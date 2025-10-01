use std::{thread, time::Duration, io::{stdout, Write}};

fn main() {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;

    // Bright ANSI colors
    let colors = [
        "\x1b[97m", // bright white
        "\x1b[96m", // bright cyan
        "\x1b[95m", // bright magenta
        "\x1b[93m", // bright yellow
        "\x1b[92m", // bright green
        "\x1b[91m", // bright red
    ];

    print!("\x1b[2J"); // Clear screen initially

    loop {
        let mut r = [' '; 1760];
        let mut z = [0.0_f64; 1760];
        let mut j: f64 = 0.0;

        while j < 6.28_f64 {
            let mut i: f64 = 0.0;
            while i < 6.28_f64 {
                let (c, l) = i.sin_cos();
                let (f, d) = j.sin_cos();
                let (e, g) = a.sin_cos();
                let (n, m) = b.sin_cos();

                let h = d + 2.0;
                let p = 1.0 / (c * h * e + f * g + 5.0);
                let t = c * h * g - f * e;

                let x = (40.0 + 30.0 * p * (l * h * m - t * n)) as usize;
                let y = (12.0 + 15.0 * p * (l * h * n + t * m)) as usize;

                let o = x + 80 * y;
                let mut q = (8.0 * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n)).abs() as usize;
                if q > 11 { q = 11; }

                if y < 22 && y > 0 && x > 0 && x < 80 && p > z[o] {
                    z[o] = p;
                    r[o] = b".,-~:;=!*#$@"[q] as char;
                }

                i += 0.02_f64;
            }
            j += 0.07_f64;
        }

        // Move cursor to top-left
        print!("\x1b[H");

        // Print donut with bright colors
        for k in 0..1760 {
            if k % 80 == 0 { print!("\n"); }
            let ch = r[k];
            if ch != ' ' {
                let idx = match ch {
                    '.' | ',' | '-' => 0,
                    '~' | ':' => 1,
                    ';' | '=' => 2,
                    '!' | '*' => 3,
                    '#' => 4,
                    '$' | '@' => 5,
                    _ => 0,
                };
                print!("{}{}", colors[idx], ch);
            } else {
                print!("{}", ch);
            }
        }

        stdout().flush().unwrap();

        // ✅ Increment rotation angles per frame
        a += 0.07;
        b += 0.03;

        thread::sleep(Duration::from_millis(30));
    }
}
