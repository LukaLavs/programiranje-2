#![allow(dead_code)]
fn main() {
    println!("Hello!");
    use BinOperacija::*;

    let expr = Izraz::Operacija(
        Box::new(Izraz::Konstanta(2)),
        Plus,
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(3)),
            Times,
            Box::new(Izraz::Konstanta(4)),
        )),
    );

    let vrednost = expr.eval();

    let stevilo_konstant = expr.collect();

    let izpis = expr.izpis();

    println!("Izraz: {}", izpis); // "(2 + (3 * 4))"
    println!("Vrednost: {}", vrednost); // 14
    println!("Število konstant: {}", stevilo_konstant); // 3

}


struct AritmeticnoZaporedje {
    zacetek: u32,
    korak: u32,
    k: i32,
}

impl AritmeticnoZaporedje { 
    fn new(zacetek: u32, korak: u32) -> Self {
        Self { zacetek, korak, k: 0 }
    }
    fn next(&mut self) -> u32 {
        self.k += 1;
        self.zacetek + ((self.k - 1) as u32) * self.korak
    }
    fn n_th(&self, n: u32) -> u32 {
        self.zacetek + (n - 1) * self.korak
    }
    fn reset(&mut self) {
        self.k = 0;
    }
    fn current(&self) -> u32 {
        if self.k == 0 {
            self.zacetek
        } else {
            self.zacetek + ((self.k - 1) as u32) * self.korak
        }
    }
    fn sum(&self, n: u32) -> u32 {
        let mut vrednost: u32 = 0;
        for i in 1..=n {
            vrednost += self.n_th(i);
        }
        vrednost
    }
    fn vsota(&self, other: &Self) -> Self {
        Self {
            zacetek: self.zacetek + other.zacetek,
            korak: self.korak + other.korak,
            k: 0,
        }
    }
}

struct GeometricnoZaporedje {
    a0: u64,
    r: u64,
    k: u32,
}

impl GeometricnoZaporedje {
    fn new(a0: u64, r: u64) -> Self {
        Self { a0, r, k: 0 }
    }
    fn next(&mut self) -> u64 {
        let val = self.a0 * self.r.pow(self.k);
        self.k += 1;
        val
    }
    fn n_th(&self, n: u32) -> u64 {
        self.a0 * self.r.pow(n - 1)
    }
    fn reset(&mut self) {
        self.k = 0;
    }
    fn current(&self) -> u64 {
        if self.k == 0 {
            self.a0
        } else {
            self.a0 * self.r.pow(self.k - 1)
        }
    }
    fn sum(&self, n: u32) -> u64 {
        let mut vrednost: u64 = 0;
        for i in 1..=n {
            vrednost += self.n_th(i);
        }
        vrednost
    }
    fn vsota(&self, other: &Self) -> Self {
        Self {
            a0: self.a0 + other.a0,
            r: self.r + other.r,
            k: 0,
        }
    }
    fn produkt(&self, other: &Self) -> Self {
        Self {
            a0: self.a0 * other.a0,
            r: self.r * other.r,
            k: 0,
        }
    }
}


enum BinOperacija {
    Plus,
    Minus,
    Times,
}
enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>), // Rekurzivni tip ima neznano velikost, zato uporabimo Box (pointer).
}

impl Izraz {
    fn eval(&self) -> u32 {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aritmeticno_zaporedje() {
        let mut a = AritmeticnoZaporedje::new(1, 2);
        assert_eq!(a.next(), 1);
        assert_eq!(a.next(), 3);
        assert_eq!(a.current(), 3);
        assert_eq!(a.n_th(4), 7);
        a.reset();
        assert_eq!(a.current(), 1);
        assert_eq!(a.sum(4), 1 + 3 + 5 + 7);

        let b = AritmeticnoZaporedje::new(2, 3);
        let mut c = a.vsota(&b);
        assert_eq!(c.zacetek, 1 + 2);
        assert_eq!(c.korak, 2 + 3);

        // Preizkus next() na vsoti
        assert_eq!(c.next(), 3); // prvi člen = zacetek + 0*korak = 3
        assert_eq!(c.next(), 8); // drugi člen = 3 + 5 = 8
    }

    #[test]
    fn test_geometricno_zaporedje() {
        let mut g = GeometricnoZaporedje::new(2, 3);
        assert_eq!(g.next(), 2);
        assert_eq!(g.next(), 6);
        assert_eq!(g.current(), 6);
        assert_eq!(g.n_th(4), 2 * 3_u64.pow(3)); // 2*27=54
        g.reset();
        assert_eq!(g.current(), 2);
        assert_eq!(g.sum(3), 2 + 6 + 18); // 2*3^0 + 2*3^1 + 2*3^2

        let h = GeometricnoZaporedje::new(5, 2);
        let p = g.produkt(&h);
        let s = g.vsota(&h);
        assert_eq!(s.a0, 2+5);
        assert_eq!(s.r, 3+2);
        assert_eq!(p.a0, 2*5);
        assert_eq!(p.r, 3*2);
    }

    #[test]
    fn test_ast_izraz() {
        use BinOperacija::*;

        let expr = Izraz::Operacija(
            Box::new(Izraz::Konstanta(2)),
            Plus,
            Box::new(Izraz::Operacija(
                Box::new(Izraz::Konstanta(3)),
                Times,
                Box::new(Izraz::Konstanta(4)),
            )),
        );

        assert_eq!(expr.eval(), 2 + 3*4); // 14
        assert_eq!(expr.collect(), 3);    // 3 konstante
        assert_eq!(expr.izpis(), "(2 + (3 * 4))");
    }
}
