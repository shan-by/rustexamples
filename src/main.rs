fn main() {
    print_complex();
    print_list();
    print_rgb();
}

//1.2.2 Display
use std::fmt;
#[derive(Debug)]
struct Complex{
    real: f32,
    imag: f32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn print_complex() {
    let point = Complex { real: 3.3, imag: 7.2};
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}

//1.2.2.1
//output:   [0: 1, 1: 2, 2: 3]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.0.iter().enumerate() {
            write!(f, "{}: {}", i, v)?;
            if i != self.0.len() - 1 { write!(f, ", ")?; }
        }
        write!(f, "]")
    }
}

fn print_list() {
    let list = List(vec![1, 2, 3]);
    println!("{}", list);
}

//1.2.3
// RGB (128, 255, 90) 0x80FF5A
// RGB (0, 3, 254) 0x0003FE
// RGB (0, 0, 0) 0x000000
struct RGB(u8, u8, u8);

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{:>0width$X}{:>0width$X}{:>0width$X}", self.0, self.1, self.2, width=2)
    }
}

fn print_rgb() {
    println!("{}", rgb1=RGB(128, 255, 90));
    println!("{}", rgb1=RGB(0, 3, 254));
    println!("{}", rgb1=RGB(0, 0, 0));
}