use std::env;
use std::io::*;

fn main() {
    let mut sin = stdin();
    let mut s = String::new();
    println!("pless enter a number");
    let conte = sin.read_line(&mut s).unwrap();

    let pointTup = prase(&mut s);
    let mut canvas = vec![vec![0i8; 50]; 50];
    drawLine(&mut canvas, pointTup);
    let mut canvas = draw(canvas);
}

//f(x,y) = ax + by + c = 0
//y = dy/dx *x + B
fn drawLine(canvas: &mut Vec<Vec<i8>>, point: (i32, i32, i32, i32)) {
    let (mut x0, mut y0, mut x1, mut y1) = point;
    let (mut a, mut b, mut d1, mut d2, mut d, mut x, mut y);
    let mut m: f32;
    if x1 < x0 {
        d = x0;
        x0 = x1;
        x1 = d;
        d = y0;
        y0 = y1;
        y1 = d;
    }
    a = y0 - y1;
    b = x1 - x0;
    if b == 0 { m = -1.0 * a as f32 * 100.0 } else { m = a as f32 / (x0 - x1) as f32 }
    x = x0;
    y = y0;
    canvas[y as usize][x as usize] = 1;

    if m >= 0.0 && m <= 1.0 {
        d = 2 * a + b;
        d1 = 2 * a;
        d2 = 2 * (a + b);
        while x < x1 {
            if d <= 0 {
                x += 1;
                y += 1;
                d += d2
            } else {
                x += 1;
                d += d1
            }
            canvas[y as usize][x as usize] = 1;
        }
    } else if m <= 0.0 && m <= -1.0 {
        d = 2 * a - b;
        d1 = 2 * a - 2 * b;
        d2 = 2 * a;
        while x < x1 {
            if d > 0 {
                x += 1;
                y -= 1;
                d += d1
            } else {
                x += 1;
                d += d2;
            }
            canvas[y as usize][x as usize] = 1;
        }
    } else if m > 1.0 {
        d = a + 2 * b;
        d1 = 2 * (a + b);
        d2 = 2 * b;
        while y < y1 {
            if d > 0 {
                x += 1;
                y += 1;
                d += d1
            } else {
                y += 1;
                d += d2;
            }
            canvas[y as usize][x as usize] = 1;
        }
    } else {
        d = a - 2 * b;
        d1 = -2 * b;
        d2 = 2 * (a - b);
        while y > y1 {
            if d <= 0 {
                x += 1;
                y -= 1;
                d += d2;
            } else {
                y -= 1;
                d += d1;
            }
        }
        canvas[y as usize][x as usize] = 1;
    }
}

fn draw(canvas: Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    for (i, ci) in canvas.iter().enumerate() {
        for (j, cj) in ci.iter().enumerate() {
            let cv = match *cj {
                0 => " ",
                1 => "*",
                _ => " ",
            };
            print!("{}", cv);
        }
        println!("--");
    }
    canvas
}


fn prase(s: &mut str) -> (i32, i32, i32, i32) {
    let sp: Vec<&str> = (&s[..s.len() - 1]).split(' ').collect();
    if sp.len() < 4 {
        panic!("Short string");
    }

    (
        sp[0].to_string().trim().parse().ok().expect("pless input number"),
        sp[1].to_string().trim().parse().ok().expect("pless input number"),
        sp[2].to_string().trim().parse().ok().expect("pless input number"),
        sp[3].to_string().trim().parse().ok().expect("pless input number")
    )
}
