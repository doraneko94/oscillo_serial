pub enum Mode {
    Plot,
    Text,
    Dev,
}

pub struct Params {
    pub mode: Mode,
    pub x_size: usize,
    pub y_size: usize,
    pub delimiter_element: String,
    pub delimiter_block: String,
    pub n_elements: usize,
}

impl Params {
    pub fn new() -> Self {
        Self {
            mode: Mode::Text,
            x_size: 50,
            y_size: 20,
            delimiter_element: ",".to_string(),
            delimiter_block: "\r\n".to_string(),
            n_elements: 1,
        }
    }

    pub fn from_args(args: &[String]) -> Self {
        let mut ret = Self::new();

        let size = args.len();
        let mut count = 1;
        loop {
            if count == size { break; }
            match args[count].as_str() {
                "-mo" => {
                    match args[count+1].as_str() {
                        "plot" => { ret.mode = Mode::Plot; }
                        "text" => { ret.mode = Mode::Text; }
                        "dev" => { ret.mode = Mode::Dev; }
                        _ => {}
                    }
                }
                "-xs" => { ret.x_size = args[count+1].parse().ok().unwrap(); }
                "-ys" => { ret.y_size = args[count+1].parse().ok().unwrap(); }
                "-de" => { ret.delimiter_element = args[count+1].to_string(); }
                "-db" => { ret.delimiter_block = args[count+1].to_string(); }
                "-ne" => { ret.n_elements = args[count+1].parse().ok().unwrap(); }
                _ => {}
            }
            count += 2;
        }

        return ret;
    }
}