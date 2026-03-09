#![allow(dead_code)]
use std::ops::{Add, Mul, Div, Sub};
use num_traits::{One, Zero, Pow};
use std::fmt::Display;

fn main() {
    println!("Hello, world!");
}



struct AritmeticnoZaporedje<T> {
    zacetek: T,
    korak: T,
    k: T,
}

impl<T> AritmeticnoZaporedje<T> 
where 
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>+ One +  Zero + PartialEq + PartialOrd,
{ 
    fn new(zacetek: T, korak: T) -> Self {
        Self { zacetek, korak, k: T::zero()}
    }
    fn next(&mut self) -> T {
        let val = self.zacetek + self.k * self.korak;
        self.k = self.k + T::one();
        val
    }
    fn n_th(&self, n: T) -> T {
        self.zacetek + self.korak * n
    }
    fn reset(&mut self) {
        self.k = T::zero()
    }
    fn current(&self) -> T {
        self.zacetek + self.k * self.korak
    }
    fn sum(&self, n: T) -> T {
        let mut vrednost: T = T::zero();
        let mut i = T::one();
        while i <= n {
            vrednost = vrednost + self.n_th(i);
            i = i + T::one();
        }
        vrednost
    }
    fn vsota(&self, other: &Self) -> Self {
        Self {
            zacetek: self.zacetek + other.zacetek,
            korak: self.korak + other.korak,
            k: T::zero(),
        }
    }
    fn produkt(&self, other: &Self) -> Self {
        Self {
            zacetek: self.zacetek * other.zacetek,
            korak: self.korak * other.korak,
            k: T::zero(),
        }
    }

    fn eq(&self, other: &Self) -> bool {
        self.zacetek == other.zacetek && self.korak == other.korak
    }
}

struct GeometricnoZaporedje<T> {
    a0: T,
    r: T,
    k: T,
}

impl<T> GeometricnoZaporedje<T>
where
    T: Copy + Mul<Output = T>  + Div<Output=T> + Log<T> + Pow<T, Output = T> + One + Zero + PartialEq + PartialOrd,
{
    fn new(a0: T, r: T) -> Self {
        Self { a0, r, k: T::zero() }
    }
    fn next(&mut self) -> T {
        let val = self.a0 * self.r.pow(self.k);
        self.k = self.k + T::one();
        val
    }
    fn n_th(&self, n: T) -> T {
        self.a0 * self.r.pow(n)
    }
    fn reset(&mut self) {
        self.k = T::zero();
    }
    fn current(&self) -> T {
        if self.k == T::zero() {
            self.a0
        } else {
            self.a0 * self.r.pow(self.k)
        }
    }
    fn sum(&self, n: T) -> T {
        let mut vrednost = T::zero();
        let mut i = T::one();
        while i <= n {
            vrednost = vrednost + self.n_th(i);
            i = i + T::one();
        }
        vrednost
    }
    fn vsota(&self, other: &Self) -> Self {
        Self {
            a0: self.a0 + other.a0,
            r: self.r + other.r,
            k: T::zero(),
        }
    }
    fn produkt(&self, other: &Self) -> Self {
        Self {
            a0: self.a0 * other.a0,
            r: self.r * other.r,
            k: T::zero(),
        }
    }
}


trait Zaporedje<T> {
    fn name(&self) -> &str;
    fn start(&self) -> T;
    fn k_th(&self, k: T) -> T;
    fn contains(&self, value: T) -> bool;
}

struct Konstantno<T> {
    value: T,
}

impl<T: Copy + PartialEq> Konstantno<T> {
    fn new(value: T) -> Self { 
        Self { value } 
    }
}

impl<T: Copy + PartialEq> Zaporedje<T> for Konstantno<T> {
    fn name(&self) -> &str { "Konstantno" }
    fn start(&self) -> T { self.value }
    fn k_th(&self, _: T) -> T { self.value }
    fn contains(&self, value: T) -> bool {
        self.value == value
    }
}


impl<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>+ One +  Zero + PartialEq + PartialOrd> 
Zaporedje<T> for AritmeticnoZaporedje<T> {
    fn name(&self) -> &str { "Aritmeticno" }
    fn start(&self) -> T { self.zacetek }
    fn k_th(&self, k: T) -> T { self.zacetek + self.korak * k }
    fn contains(&self, value: T) -> bool {
        let potential_k = (value - self.zacetek) / self.korak;
        if self.n_th(potential_k) == value && potential_k >= T::zero() {
            return true;
        }
        return false
    }
}

impl< T: Copy + Mul<Output = T>  + Div<Output=T> + Log<T> + Pow<T, Output = T> + One + Zero + PartialEq + PartialOrd> 
Zaporedje<T> for GeometricnoZaporedje<T> {
    fn name(&self) -> &str { "Geometricno" }
    fn start(&self) -> T { self.a0 }
    fn k_th(&self, k: T) -> T { self.a0 * self.r.pow(k) }
    fn contains(&self, value: T) -> bool {
        let potential_n = (value / self.a0).ln() / self.r.ln();
        if self.k_th(potential_n) == value && potential_n >= T::zero() {
            return true;    
        }   
        return false
    }
}


struct Zamaknjeno<T> {
    base: Box<dyn Zaporedje<T>>,
    offset: T,
}

impl<T: Copy + Add<Output = T>> Zamaknjeno<T> {
    fn new(base: Box<dyn Zaporedje<T>>, offset: T) -> Self {
        Self { base, offset }
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> Zaporedje<T> for Zamaknjeno<T> {
    fn name(&self) -> &str { "Zamaknjeno" }
    fn start(&self) -> T { self.base.start() + self.offset }
    fn k_th(&self, k: T) -> T { self.base.k_th(k) + self.offset }
    fn contains(&self, value: T) -> bool {
        self.base.contains(value - self.offset)
    }
}





enum BinOperacija {
    Plus,
    Minus,
    Times,
}
enum Izraz<T: Display> {
    Konstanta(T),
    Operacija(Box<Izraz<T>>, BinOperacija, Box<Izraz<T>>), // Rekurzivni tip ima neznano velikost, zato uporabimo Box (pointer).
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialEq + PartialOrd + Display> Izraz<T> {
    fn eval(&self) -> T {
        match self {
            Izraz::Konstanta(v) => *v,
            Izraz::Operacija(levo, op, desno) => {
                let l = levo.eval();
                let r = desno.eval();
                match op {
                    BinOperacija::Plus => l + r,
                    BinOperacija::Minus => l - r,
                    BinOperacija::Times => l * r,
                }
            }
        }
    }
    fn collect(&self) -> i32 {
        match self {
            Izraz::Konstanta(_) => 1,
            Izraz::Operacija(levo, _, desno) => {
                levo.collect() + desno.collect()
            }
        }
    }
    fn izpis(&self) -> String {
        match self {
            Izraz::Konstanta(v) => format!("{}", v),
            Izraz::Operacija(levo, op, desno) => {
                let op_str = match op {
                    BinOperacija::Plus => "+",
                    BinOperacija::Minus => "-",
                    BinOperacija::Times => "*",
                };
                format!("({} {} {})", levo.izpis(), op_str, desno.izpis())
            }
        }
    }

}


trait Log<T> {
    fn ln(&self) -> Self;
}

impl Log<f32> for f32 {
    fn ln(&self) -> Self { f32::ln(*self) }
}


// TODO: Implement Combined struct using Izraz to represent combined sequences.
// TODO: Write tests for all implemented functionality. Test with matrixes and general algebraic objects.
