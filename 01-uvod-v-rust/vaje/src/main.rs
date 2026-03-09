use core::panic;

/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(a0: u32, a1: u32, n: u32) -> u32 {
    let mut prev = a0;
    let mut curr = a1; 
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    if n == 0 { a0 } else { curr }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

fn je_prestopno(leto: u32) -> bool {
    (leto % 4 == 0 && leto % 100 != 0) || (leto % 400 == 0)
}
/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);

fn je_veljaven_datum((dan, mesec, leto): Date) -> bool {
    if mesec < 1 || mesec > 12 {
        return false;
    }
    let dni_v_mesecu = match mesec {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if je_prestopno(leto) { 29 } else { 28 },
        _ => return false,
    };
    dan >= 1 && dan <= dni_v_mesecu
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

fn iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    loop { 
        if cond(start) { break start; } 
        start = fun(start);
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b] glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

fn bisekcija(mut a: f64, mut b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    loop { 
        let c = (a + b) / 2.0;
        let fc = fun(c);
        if fc.abs() < prec || (b - a).abs() < prec {
            break c;
        }
        if fun(a) * fc < 0.0 { b = c; } else { a = c; }
    }
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.

// fn guessing_game() {
//     panic!("Not implemented");
// }

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    let mut result = [[0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    let asc = arr.windows(2).all(|w| w[0] <= w[1]);
    let desc = arr.windows(2).all(|w| w[0] >= w[1]);
    asc || desc
}

// fn vsebuje<T : PartialEq>(v: &Vec<T>, x : &T) -> bool {
//     for y in v {
//       if x == y {
//         return true
//       }
//     }
//     return false
// }

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`

fn pow(x: u32, n: u32) -> u32 {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        let half = pow(x, n / 2);
        half * half
    } else {
        x * pow(x, n - 1)
    }
}

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

fn pow_iter(mut x: u32, mut n: u32) -> u32 { 
    let mut result = 1;
    while n > 0 {
        if n % 2 == 1 {
            result *= x;
        }
        x *= x;
        n /= 2;
    }
    result
}
/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32 { 
    let mut result = 1 % m;
    x = x % m;
    while n > 0 {
        if n % 2 == 1 {
            result = (result * x) % m;
        }
        x = (x * x) % m;
        n /= 2;
    }
    result
}
/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(arr: &mut [u32]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;
        for j in (i + 1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn pyramid(n: u32) {
    let width = 2 * n - 1;
    for i in 0..n {
        let stars = 2 * i + 1;
        let spaces = (width - stars) / 2;
        println!("{}{}", " ".repeat(spaces as usize), "*".repeat(stars as usize));
    }
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahkom predpostavite, da bo n največ 26.
///      A
///    A B A
///   A B C B A
/// A B C D C B A


fn pyramid_letters(n: u32) {
    for i in 0..n {
        let mut line = String::new();
        for _ in 0..(2*n - 2*i - 1) {
            line.push(' ');
        }
        for j in 0..=i {
            line.push((b'A' + j as u8) as char);
            if j != i {
                line.push(' ');
            }
        }
        for j in (0..i).rev() {
            line.push(' ');
            line.push((b'A' + j as u8) as char);
        }
        println!("{}", line);
    }
}

fn main() {
    // ---------------- Fibonacci ----------------
    println!("fib(0, 1, 6) = {}", fib(0, 1, 6));
    println!("fib(2, 3, 4) = {}", fib(2, 3, 4));

    // ---------------- Prestopno leto ----------------
    println!("2020 prestopno? {}", je_prestopno(2020));
    println!("2019 prestopno? {}", je_prestopno(2019));

    // ---------------- Datum ----------------
    let d1: Date = (29, 2, 2020);
    let d2: Date = (29, 2, 2019);
    println!("Datum {:?} veljaven? {}", d1, je_veljaven_datum(d1));
    println!("Datum {:?} veljaven? {}", d2, je_veljaven_datum(d2));

    // ---------------- Iteracija ----------------
    fn inc(x: u32) -> u32 { x + 1 }
    fn cond(x: u32) -> bool { x >= 10 }
    println!("iteracija od 0 do >=10 = {}", iteracija(0, inc, cond));

    // ---------------- Bisekcija ----------------
    fn f(x: f64) -> f64 { x * x - 2.0 }
    let root = bisekcija(1.0, 2.0, f, 1e-6);
    println!("ničla x^2 - 2 ≈ {}", root);

    // ---------------- Matrike ----------------
    let a = [[1, 2], [3, 4]];
    let b = [[5, 6], [7, 8]];
    println!("mat_mul(a, b) = {:?}", mat_mul(a, b));

    // ---------------- Ordered ----------------
    println!("ordered [1,2,3] = {}", ordered(&[1, 2, 3]));
    println!("ordered [3,2,1] = {}", ordered(&[3, 2, 1]));
    println!("ordered [1,3,2] = {}", ordered(&[1, 3, 2]));

    // ---------------- Potenciranje ----------------
    println!("pow(2, 10) = {}", pow(2, 10));
    println!("pow_iter(2, 10) = {}", pow_iter(2, 10));
    println!("pow_mod(2, 10, 1000) = {}", pow_mod(2, 10, 1000));

    // ---------------- Selection sort ----------------
    let mut arr = [3, 1, 4, 1, 5];
    println!("pred sortiranjem: {:?}", arr);
    selection_sort(&mut arr);
    println!("po sortiranju:    {:?}", arr);

    // ---------------- Piramide ----------------
    println!("\nPiramida zvezdic:");
    pyramid(4);

    println!("\nPiramida črk:");
    pyramid_letters(4);
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(0, 1, 0), 0);
        assert_eq!(fib(0, 1, 1), 1);
        assert_eq!(fib(0, 1, 5), 5);
        assert_eq!(fib(2, 3, 4), 13);
    }

    #[test]
    fn test_prestopno() {
        assert!(je_prestopno(2020));
        assert!(!je_prestopno(2019));
        assert!(je_prestopno(2000));
        assert!(!je_prestopno(1900));
    }

    #[test]
    fn test_veljaven_datum() {
        assert!(je_veljaven_datum((29, 2, 2020)));
        assert!(!je_veljaven_datum((29, 2, 2019)));
        assert!(!je_veljaven_datum((31, 4, 2023)));
        assert!(je_veljaven_datum((31, 12, 2023)));
    }

    #[test]
    fn test_iteracija() {
        fn f(x: u32) -> u32 { x + 1 }
        fn cond(x: u32) -> bool { x >= 10 }

        assert_eq!(iteracija(0, f, cond), 10);
    }

    #[test]
    fn test_bisekcija() {
        fn f(x: f64) -> f64 { x * x - 2.0 }

        let root = bisekcija(1.0, 2.0, f, 1e-6);
        assert!((root - 2.0_f64.sqrt()).abs() < 1e-6);
    }

    #[test]
    fn test_mat_mul() {
        let a = [[1, 2], [3, 4]];
        let b = [[5, 6], [7, 8]];
        let res = mat_mul(a, b);

        assert_eq!(res, [[19, 22], [43, 50]]);
    }

    #[test]
    fn test_ordered() {
        assert!(ordered(&[1, 2, 2, 3]));
        assert!(ordered(&[5, 4, 3, 1]));
        assert!(!ordered(&[1, 3, 2]));
        assert!(ordered(&[]));
        assert!(ordered(&[42]));
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow(2, 0), 1);
        assert_eq!(pow(2, 10), 1024);
    }

    #[test]
    fn test_pow_iter() {
        assert_eq!(pow_iter(3, 4), 81);
    }

    #[test]
    fn test_pow_mod() {
        assert_eq!(pow_mod(2, 10, 1000), 24);
        assert_eq!(pow_mod(3, 0, 7), 1);
    }

    #[test]
    fn test_selection_sort() {
        let mut arr = [3, 1, 4, 1, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 1, 3, 4, 5]);
    }
}
