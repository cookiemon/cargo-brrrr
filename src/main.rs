use terminal_size::terminal_size;

static CAR: &str = include_str!("car.txt");

fn print_car(x: i32) {
    let term_sz = terminal_size().unwrap();
    let term_sz_x = term_sz.0.0 as usize;

    if x < 0 {
        let cut = -x as usize;

        for ln in CAR.lines() {
            let low = cut.min(ln.len());
            let high = (cut + term_sz_x).min(ln.len());
            println!("{}", &ln[low..high]);
        }
    } else if (x as usize) < term_sz_x {
        let x = x as usize;

        for ln in CAR.lines() {
            let high = (term_sz_x - x).min(ln.len());
            print!("{}", " ".repeat(x));
            println!("{}", &ln[..high]);
        }
    } else {
        for _ in CAR.lines() {
            println!();
        }
    }
}

fn reset() {
    let linecount = CAR.lines().count();
    let up = format!("\u{1B}[{}F", linecount);
    print!("{}", up);
}

fn main() {
    ctrlc::set_handler(|| {
        // No! U watch dis!
    }).ok(); // (Except when can't force you, then whatever)

    let term_sz_x = terminal_size().unwrap().0.0 as i32;
    let car_size = CAR.lines().map(str::len).max().unwrap() as i32;
    for x in -car_size..term_sz_x {
        print_car(x);
        let frame_time = std::time::Duration::from_millis(10);
        std::thread::sleep(frame_time);
        reset();
    }
}
