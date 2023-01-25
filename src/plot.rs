use ndarray::*;

pub struct PointUsize { pub y: usize, pub x: usize }

impl PointUsize {
    pub fn new(y: usize, x: usize) -> Self { Self { y, x } }
    pub fn to_f32(&self) -> PointF32 { PointF32::new(self.y as f32, self.x as f32) }
}

pub struct PointF32 { pub y: f32, pub x: f32 }

impl PointF32 {
    pub fn new(y: f32, x: f32) -> Self { Self { y, x } }
    pub fn _to_usize(&self) -> PointUsize { PointUsize::new(self.y as usize, self.x as usize) }
}

pub fn draw_line(disp: &mut Array2<i8>, c: i8, p0: (usize, usize), p1: (usize, usize)) {
    if p0.0 == p1.0 {
        if p0.1 == p1.1 { disp[[p0.0, p0.1]] = c; }
        else if p0.1 < p1.1 { for xi in p0.1..p1.1+1 { disp[[p0.0, xi]] = c; } }
        else { for xi in p1.1..p0.1+1 { disp[[p0.0, xi]] = c; } }
        return;
    } else if p0.1 == p1.1 {
        if p0.0 < p1.0 { for yi in p0.0..p1.0+1 { disp[[yi, p0.1]] = c; } }
        else { for yi in p1.0..p0.0+1 { disp[[yi, p0.1]] = c; } }
        return;
    }
    let (pi0, pi1)= if p0.1 < p1.1 {
        (PointUsize::new(p0.0, p0.1), PointUsize::new(p1.0, p1.1))
    } else {
        (PointUsize::new(p1.0, p1.1), PointUsize::new(p0.0, p0.1))
    };
    let (pf0, pf1) = (pi0.to_f32(), pi1.to_f32());
    let a = (pf1.y - pf0.y) / (pf1.x - pf0.x);
    let b = pf0.y - a * pf0.x;
    let mut ni = PointUsize::new(pi0.y, pi0.x);
    
    while ni.y != pi1.y || ni.x != pi1.x {
        disp[[ni.y, ni.x]] = c;
        let nf = ni.to_f32();
        if ni.x < pi1.x {
            let y = a * (nf.x + 1.0) + b;
            let y_diff = y - nf.y;
            if y_diff <= 1.5 && y_diff >= -1.5 {
                ni.x += 1;
                if y_diff > 0.5 { ni.y += 1; }
                else if y_diff < -0.5 { ni.y -= 1; }
                continue;
            }
        }
        if ni.y < pi1.y && pi0.y < pi1.y {
            let x = ((nf.y + 1.0) as f32 - b) / a;
            let x_diff = x - nf.x;
            if x_diff <= 1.5 && x_diff >= -0.5 {
                ni.y += 1;
                if x_diff > 0.5 { ni.x += 1; }
                continue;
            }
        } else if ni.y > pi1.y && pi0.y > pi1.y {
            let x = ((nf.y - 1.0) as f32 - b) / a;
            let x_diff = x - nf.x;
            if x_diff <= 1.5 && x_diff >= -0.5 {
                ni.y -= 1;
                if x_diff > 0.5 { ni.x += 1; }
                continue;
            }
        }
        break;
    }
    disp[[pi1.y, pi1.x]] = c;
}