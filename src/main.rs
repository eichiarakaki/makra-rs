#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_macros)]
// #![allow(unused_mut)]
// #![allow(unused_variables)]

struct Input {
    data: Vec<String>,
}

impl Input {
    pub fn new() -> Input {
        Input { data: Vec::new() }
    }

    fn read(&mut self) {
        let mut line: String = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        self.data = line
            .split_whitespace()
            .rev()
            .map(|s| s.parse::<String>().unwrap())
            .collect::<Vec<_>>();
    }

    pub fn get<T: core::str::FromStr>(&mut self) -> T {
        if self.data.len() == 0 {
            self.read();
        }
        self.data.pop().unwrap().parse::<T>().ok().unwrap()
    }
}

impl<T: core::str::FromStr> std::ops::Shr<&mut T> for Input {
    type Output = Input;

    fn shr(mut self, rhs: &mut T) -> Input {
        *rhs = self.get::<T>();
        self
    }
}

macro_rules! ift {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    };
}

trait SingleNMath<T> {
    fn is_prime(self) -> bool;
    fn factorial(self) -> T;
}

macro_rules! SingleNMath {
    ($type:ty) => {
        impl SingleNMath<$type> for $type {
            // O(1)
            fn is_prime(self) -> bool {
                let last = (self as f64).sqrt() as i64 + 1;
                x > 1 && (2..last).all(|d| x % d != 0)
            }
            // O(nk)
            fn factorial(self) -> $type {
                (1..=self).into_iter().fold(1 as $type, |acc, n| acc * n)
            }
        }
    };
}
// unsigned integers
SingleNMath!(u8);
SingleNMath!(u16);
SingleNMath!(u32);
SingleNMath!(u64);
SingleNMath!(u128);
SingleNMath!(usize);

trait MathCalc<T> {
    fn median(&mut self) -> T;
    fn average(&self) -> f32;
}

macro_rules! MathCalcs {
    ($type:ty) => {
        impl MathCalc<$type> for Vec<$type> {
            // O(3)
            fn median(&mut self) -> $type {
                self.sort();
                let mid = self.len() / 2;
                self[mid]
            }
            // O(n+2)
            fn average(&self) -> f32 {
                self.iter().sum::<$type>() as f32 / self.len() as f32
            }
        }
    };
}
// unsigned integers
MathCalcs!(u8);
MathCalcs!(u16);
MathCalcs!(u32);
MathCalcs!(u64);
MathCalcs!(u128);
MathCalcs!(usize);
// signed integers
MathCalcs!(i8);
MathCalcs!(i16);
MathCalcs!(i32);
MathCalcs!(i64);
MathCalcs!(i128);
MathCalcs!(isize);

fn test_case() {
    let mut rin = Input::new();
    let mut times = 0;
    // rin = rin >> &mut times;

    // for _ in 0..times {}
}

fn main() {
    // let mut rin = Input::new();
    // let mut out: u32 = 0;

    // rin = rin >> &mut out;
    // for i in 1..out + 1 {
    //     print!("Case #{i}: ");
    //     test_case();
    // }
    test_case();
}
