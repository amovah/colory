extern crate rand;

use self::rand::Rng;

fn maximum(numbers: Vec<f32>) -> f32 {
    let mut max:f32 = 0.0;
    for num in numbers {
        if num > max {
            max = num;
        }
    }
    max
}

fn minimum(numbers: Vec<f32>) -> f32 {
    let mut min:f32 = 0.0;
    for num in numbers {
        if num < min {
            min = num;
        }
    }
    min
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn log(&self) {
        println!("Red: {}, Green: {}, Black: {}", self.r, self.g, self.b);
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let b = self.b as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let max:f32 = maximum(vec![r, g, b]);
        let min:f32 = minimum(vec![r, g, b]);

        let l:f32  = (max + min ) / 2.0;
        let mut h:f32 = 0.0;
        let mut s:f32 = 0.0;
        if max == min {
            h = 0.0;
            s = 0.0;
        } else {
            let d = max - min;
            if l > 0.5 {
                s = d / (2.0 - max - min);
            } else {
                s = d / (max - min);
            }

            if max == r {
                if g < b {
                    h = (g - b) / d + 6.0;
                } else {
                    h = (g - b) / d;
                }
            }

            if max == g {
                h = (b - r) / d + 2.0;
            }

            if max == b {
                h = (r - g) / d + 4.0;
            }

            h = h / 6.0;
        }

        (h, s, l)
    }
}

pub struct Generator {
    r: u8,
    g: u8,
    b: u8
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            r: 0,
            g: 0,
            b: 0
        }
    }

    pub fn r(mut self, r: u8) -> Generator {
        self.r = r;
        self
    }

    pub fn g(mut self, g: u8) -> Generator {
        self.g = g;
        self
    }

    pub fn b(mut self, b: u8) -> Generator {
        self.b = b;
        self
    }

    pub fn generate(self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b
        }
    }

    pub fn random() -> Color {
        Color {
            r: rand::thread_rng().gen_range(0, 255),
            g: rand::thread_rng().gen_range(0, 255),
            b: rand::thread_rng().gen_range(0, 255),
        }
    }
}
