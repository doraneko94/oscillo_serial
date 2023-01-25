use std::io::{self, stdout, Read};

use crossterm::{execute, terminal, cursor};
use ndarray::*;

use oscillo_serial::exp::ManExp;
use oscillo_serial::params::{Params, Mode};
use oscillo_serial::plot::draw_line;
use oscillo_serial::utils::min_max;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let params = Params::from_args(&args);
    let mut stdout = stdout();

    match params.mode {
        Mode::Dev => {}
        _ => {}
    }

    for port in serialport::available_ports()? {
        match serialport::new(port.port_name, 115200)
            .timeout(std::time::Duration::from_millis(10))
            .flow_control(serialport::FlowControl::Hardware)
            .open()
        {
            Ok(mut port) => {
                let mut buf = [0; 1024];
                match params.mode {
                    Mode::Dev => {}
                    Mode::Plot => {
                        let mut data: Array2<f32> = Array2::zeros((params.n_elements, params.x_size));
                        let mut data_tmp: Array1<f32> = Array1::zeros(params.n_elements);
                        let mut count = 0;
                        let mut s_save = String::new();
                        loop {
                            match port.read(&mut buf) {
                                Ok(t) => {
                                    let mut new_data = false;
                                    let s = String::from_utf8(buf[..t].to_vec())?;
                                    let blocks_len = s.split(&params.delimiter_block).count();
                                    for (bi, block) in s.split(&params.delimiter_block).enumerate() {
                                        s_save = s_save + &block;
                                        if bi < blocks_len - 1 {
                                            let elements_len = s_save.split(&params.delimiter_element).count();
                                            if elements_len == params.n_elements {
                                                let mut valid = true;
                                                for (ei, elem) in s_save.split(&params.delimiter_element).enumerate() {
                                                    match elem.parse().ok() {
                                                        Some(val) => { data_tmp[ei] = val; }
                                                        None => {
                                                            valid = false;
                                                            break;
                                                        }
                                                    }
                                                }
                                                if valid {
                                                    Zip::from(data.slice_mut(s![.., count%params.x_size]))
                                                        .and(&data_tmp)
                                                        .for_each(|a, &b| *a = b);
                                                    count += 1;
                                                    new_data = true;
                                                }
                                            }
                                            s_save = String::new();
                                        }
                                    }
                                    if new_data {
                                        // clear terminal
                                        execute!(
                                            stdout,
                                            cursor::MoveTo(0, 0),
                                            terminal::Clear(terminal::ClearType::All)).unwrap();
                                        update(&data, count, params.y_size)?;
                                    }
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                                Err(e) => return Err(e.into()),
                            }
                        }
                    }
                    Mode::Text => {
                        loop {
                            match port.read(&mut buf) {
                                Ok(t) => {
                                    // clear terminal
                                    println!("{}", String::from_utf8(buf[..t].to_vec()).unwrap());
                                    
                                    //io::stdout().write_all(&mut buf[..t]);
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                                Err(e) => return Err(e.into()),
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    };
    Ok(())
}

fn update(data: &Array2<f32>, count: usize, y_size: usize) -> Result<(), Box<dyn std::error::Error>> {
    
    let (min, max) = min_max(&data, count);
    let interval = max - min;
    let (low, high) = if interval == 0.0 {
        (min*0.9, min*1.1)
    } else {
        let margin = interval * 0.1;
        (min - margin, max + margin)
    };
    let shape = data.shape();
    let (n_elements, x_size) = (shape[0], shape[1]);
    let y_unit = (high - low) / (y_size - 1) as f32;

    let mut disp = Array2::zeros((y_size, x_size)) - 1;
    if count >= x_size {
        let start = count % x_size;
        for xi in 0..x_size-1 {
            let x0 = (xi + start) % x_size;
            let x1 = (xi + 1 + start) % x_size;
            for ei in 0..n_elements {
                let y0 = ((data[[ei, x0]] - low) / y_unit) as usize;
                let y1 = ((data[[ei, x1]] - low) / y_unit) as usize;
                draw_line(&mut disp, ei as i8, (y0, xi), (y1, xi+1));
            }
        }
    } else {
        let x_unit = (count - 1) as f32 / (x_size - 1) as f32;
        for xi in 0..count-1 {
            let x0 = (xi as f32 / x_unit) as usize;
            let x1 = ((xi+1) as f32 / x_unit) as usize;
            for ei in 0..n_elements {
                let y0 = ((data[[ei, xi]] - low) / y_unit) as usize;
                let y1 = ((data[[ei, xi+1]] - low) / y_unit) as usize;
                draw_line(&mut disp, ei as i8, (y0, x0), (y1, x1));
            }
        }
    }
    let mut s;
    
    let mut head = ManExp::zero();
    let (mut hi, mut lo) = (ManExp::from_f32(high), ManExp::from_f32(low));
    let int = hi - lo;
    if int.to_f32() < lo.to_f32().abs() * 0.01 {
        head = lo;
        hi -= lo;
        lo = ManExp::zero();
    }
    let base_exp = int.to_f32().log10().floor() as i32;
    let mut base = ManExp::new(1, base_exp);
    let mut start;
    let mut count = 0;
    loop {
        start = lo.div_ceil(&base) * base;
        let tmp = (hi - start).div_floor(&base).to_f32();
        count += 1;
        if count >= 10 {
            break;
        }
        if  tmp > 5f32 {
            base = base.double();
            continue;
        }
        if tmp < 2f32 {
            base = base.half();
            continue;
        }
        break;
    }
    let mut ticks = Vec::new();
    let mut now = start;
    while now.to_f32() <= hi.to_f32() {
        ticks.push(now);
        now += base;
    }
    
    let mut tick_labels = vec!["        ".to_string(); y_size];
    for &tick in ticks.iter() {
        tick_labels[y_size - 1 - ((tick - lo).to_f32() / y_unit) as usize] = tick.to_string();
    }
    if head.is_zero() {
        println!("(+       0)");
    } else {
        if head.is_neg() {
            print!("( "); print!("{}", head.to_string()); println!(")");
        } else {
            print!("(+"); print!("{}", head.to_string()); println!(")");
        }
    }
    for yi in 0..y_size {
        print!("{}", tick_labels[yi]);
        if tick_labels[yi] == "        ".to_string() {
            print!("|");
        } else {
            print!("-");
        }
        s = String::new();
        for xi in 0..x_size {
            match disp[[yi, xi]] {
                -3 => { s = s + "|" }
                -2 => { s = s + "-" }
                -1 => { s = s + " " }
                n => {
                    print!("{}", s);
                    s = String::new();
                    let n_mod = n % 6;
                    if n_mod == 0 { print!("{}", colored::Colorize::red("@")); }
                    else if n_mod == 1 { print!("{}", colored::Colorize::green("@")); }
                    else if n_mod == 1 { print!("{}", colored::Colorize::yellow("@")); }
                    else if n_mod == 1 { print!("{}", colored::Colorize::blue("@")); }
                    else if n_mod == 1 { print!("{}", colored::Colorize::magenta("@")); }
                    else  { print!("{}", colored::Colorize::cyan("@")); }
                }
            }
        }
        println!("{}", s);
    }
    Ok(())
}