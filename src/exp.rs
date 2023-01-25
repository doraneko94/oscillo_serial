use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

#[derive(Clone, Copy)]
pub struct ManExp {
    pub man: i32,
    pub exp: i32,
}

impl ManExp {
    pub fn new(man: i32, exp: i32) -> Self {
        let mut ret = Self { man, exp };
        ret._scale();
        ret
    }
    fn _scale(&mut self) {
        if self.man == 0 {
            self.exp = 0;
            return;
        }
        while self.man.abs() >= 1000000 {
            self.man /= 10;
            self.exp += 1;
        }
        while self.man.abs() < 100000 {
            self.man *= 10;
            self.exp -= 1;
        }
    }
    pub fn from_f32(f: f32) -> Self {
        if f == 0f32 { return ManExp::new(0, 0); }
        let exp = f.abs().log10().floor() as i32;
        let man = (f * 10f32.powi(-exp + 6)) as i32;
        let mut ret = Self::new(man, exp - 6);
        ret._scale();
        ret
    }
    pub fn to_f32(&self) -> f32 {
        self.man as f32 * 10f32.powi(self.exp)
    }
    pub fn zero() -> Self {
        Self::new(0, 0)
    }
    pub fn is_zero(&self) -> bool { self.man == 0 }
    pub fn is_neg(&self) -> bool { self.man < 0 }
    pub fn div_floor(&self, other: &Self) -> Self {
        if self.man == 0 { return Self::new(0, 0); }
        let num = self.man.abs() as u64 * 1000000;
        let den = other.man.abs() as u64;
        let mut val = (num / den) as i32;
        if (self.man < 0 && other.man > 0) || (self.man > 0 && other.man < 0) {
            val = -val;
        }
        Self::new(val, self.exp-6-other.exp)
    }
    pub fn div_ceil(&self, other: &Self) -> Self {
        if self.man == 0 { return Self::new(0, 0); }
        let num = self.man.abs() as u64 * 1000000;
        let den = other.man.abs() as u64;
        let mut val = (num / den) as i32;
        if num % den != 0 { val += 1000000; }
        if (self.man < 0 && other.man > 0) || (self.man > 0 && other.man < 0) {
            val = -val;
        }
        Self::new(val, self.exp-6-other.exp)
    }
    pub fn half(&self) -> Self {
        let mut ret = Self::new(self.man * 5, self.exp - 1);
        ret._scale();
        ret
    }
    pub fn double(&self) -> Self {
        let mut ret = Self::new(self.man * 2, self.exp);
        ret._scale();
        ret
    }
    pub fn to_string(&self) -> String {
        if self.man == 0 { return "       0".to_string(); }
        let (mut ret, mut count) = if self.man < 0 {
            ("-".to_string(), 1)
        } else {
            (" ".to_string(), 0)
        };
        let s: Vec<char> = self.man.to_string().chars().collect();
        if self.exp <= -1 && self.exp >= -8 {
            if self.exp >= -5 {
                for i in 0..7 {
                    if i == 6 + self.exp {
                        ret = ret + ".";
                    } else {
                        ret = ret + &s[count].to_string();
                        count += 1;
                    }
                }
            } else {
                ret = ret + "0.";
                for i in 0..5 {
                    if i < -self.exp - 6 {
                        ret = ret + "0";
                    } else {
                        ret = ret + &s[count].to_string();
                        count += 1;
                    }
                }
            }
        } else {
            let rank = self.exp + 5;
            ret = ret + &s[count].to_string() + "." + &s[count+1].to_string() + "E";
            ret += if rank < 0 { "-" } else { "0" };
            let rank_abs = rank.abs().to_string();
            if rank_abs.len() == 2 {
                ret = ret + &rank_abs;
            } else {
                ret = ret + "0" + &rank_abs;
            };
        }
        ret
    }
}

impl Neg for ManExp {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.man, self.exp)
    }
}
impl Add for ManExp {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.man == 0 {
            return rhs;
        }
        if rhs.man == 0 {
            return self;
        }
        if self.exp >= rhs.exp + 5 {
            Self::new(self.man, self.exp)
        } else if self.exp + 5 <= rhs.exp {
            Self::new(rhs.man, rhs.exp)
        } else {
            if self.exp == rhs.exp {
                let mut ret = Self::new(self.man+rhs.man, self.exp);
                ret._scale();
                ret
            } else if self.exp > rhs.exp {
                let powu = (self.exp - rhs.exp) as u32;
                let mut ret = Self::new(self.man+rhs.man/10i32.pow(powu), self.exp);
                ret._scale();
                ret
            } else {
                let powu = (rhs.exp - self.exp) as u32;
                let mut ret = Self::new(self.man/10i32.pow(powu)+rhs.man, rhs.exp);
                ret._scale();
                ret
            }
        }
    }
}
impl AddAssign for ManExp {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for ManExp {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}
impl SubAssign for ManExp {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl Mul for ManExp {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.man == 0 || rhs.man == 0 { return Self::new(0, 0); }
        let mut ret = Self::new((self.man/100)*(rhs.man/100), self.exp+rhs.exp+4);
        ret._scale();
        ret
    }
}
impl MulAssign for ManExp {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}