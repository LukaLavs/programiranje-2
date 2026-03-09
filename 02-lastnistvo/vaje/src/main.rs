use std::time::{Duration, Instant};

#[allow(dead_code)]
fn time_it<F: FnOnce() -> R, R>(f: F) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

#[allow(unused_mut)]
fn on_stack() {
    // Narišite shemo spreminjanja sklada in kopice
    // Za vsako vrstico napiši, kolikokrat se v pomnilniku pojavi 13?
    let mut a = [13; 100]; // 100 krat na skladu
    let mut b = a; // 100 krat na skladu, ker je kopirano
    let q = String::from("13"); // 1 krat na kopici ter pointer na skladu
    println!("{}", q); 
    let r = q; // premik na drugo mesto, še vedno 1 krat na kopici
    let p = &r; // pointer na skladu
    a[0] = 1; // ena 13 manj na skladu
    {
        let c = &b; // pointer na skladu
        println!("{}", c[0]); // še vedno 100 krat na skladu
    } // pointer c izgine
    println!("{}", b[0]); // še vedno 100 krat na skladu
    println!("{}", a[0]); // še vedno 99 krat na skladu
    println!("{}", p); // še vedno 1 krat na kopici
    println!("{}", r); // še vedno 1 krat na kopici
    // println!("{}", q); // Razloži, zakaj to ne deluje. q je bil premaknjen v r
}

/// Napišite funkcijo `swap`, ki zamenja vrednosti dveh celoštevilskih spremenljivk.
fn test_swap() {
    // V spremenljivko `a` shranite vrednost 13, v spremenljivko `b` pa vrednost 42.
    let mut a = 13;
    let mut b = 42;
    println!("a: {}, b: {}", a, b);
    // Izpiše `a: 13, b: 42`.

    // Naredite swap s pomočjo pomožne funkcije `swap`.

    // Način 1: 
        // fn swap(x: &mut i32, y: &mut i32) {
        //     let temp = *x;
        //     *x = *y;
        //     *y = temp;
        // }
        // swap(&mut a, &mut b);

    // Način 2: 
    std::mem::swap(&mut a, &mut b);
    println!("a: {}, b: {}", a, b);
    // Izpiše `a: 42, b: 13`.
}

/// Popravite zakomentiran del spodnje funkcije, da bo deloval
fn str_own() {
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}", x, y);
}

/// Popravite brez uporabe funkcije `clone`
/// Namig: sklad in kopiranje na skladu - kodo lahko spremenite
fn str_own2() {
    let x = (1, 2, (), String::from("Hello world"));
    let y = &x;
    println!("{:?}, {:?}", x, y);
}

/// Popravite spodnji dve funkciji, da bosta delovali

fn wrong() {
    let s = String::from("Hello World");
    print_str(&s);
    println!("{}", s);
}

fn print_str(s: &String) {
    println!("{}", s)
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala
fn fn1() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala

fn fn2() {
    let x = Box::new(5);

    // Popravite zgolj tukaj vmes
   let mut y = Box::new(*x);
    //
    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------

fn fn3() {
    let t = (
        String::from("hello"),
        String::from("world"),
        String::from("!"),
    );

    let _s = &t.1;

    // Izpišite čim večji del t-ja.
    println!("({}, {}, {})", t.0, t.1, t.2);
}

/// ------------------------------------------------------------------------------------------------

fn fn4() {
    let x = 5;
    // Izpišite naslov spremenljivke x
    println!("Naslov za x: {:p}", &x);
}

/// ------------------------------------------------------------------------------------------------

fn fn5() {
    let x = 13;
    let y = &x;

    // Popravite spodnjo vrstico, da bo bo enakost držala
    assert_eq!(13, *y);
}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper` se mora poklicati čim bolj učinkovito.
fn fn6() {
    let s = String::from("hello, ");

    helper(&s);

    println!("Success!");
}

// Te funkcije ne spreminjajte
#[allow(unused_variables)]
fn helper(s: &String) {}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper2` se mora poklicati čim bolj učinkovito.
fn fn7() {
    let mut s = String::from("hello, ");

    helper2(&mut s);

    println!("Success!");
}
// Te funkcije ne spreminjajte
fn helper2(s: &mut String) {
    s.push_str("world")
}

/// ------------------------------------------------------------------------------------------------

/// Pojasnite, zakaj spodnja koda ne deluje
fn fn8() {
    // let mut s = String::from("hello, ");

    // let p = &mut s;

    // p.push_str("world");

    // println!("Success! {}", p); // println! ne deluje, ker ne smemo imeti referenco
    //                            // do s in hkrati imeti mutabilno referenco p do s
    // println!("Success! {}", s); // ne deluje iz istega razloga
    // p.push_str("!");
}

/// ------------------------------------------------------------------------------------------------
/// Pojasnite, zakaj spodnja koda ne deluje in jo popravite
/// Pojasnite tudi zakaj je popravek ok

fn fn9() {
    let mut s = String::from("hello");

    let r1 = s.clone();
    // let r2 = &mut s; // obstaja lahko le ena mutabilna referenca na spremenljivko v določenem času

    let r2 = &mut s;
    println!("{}, {}", r2, r1);

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------
fn fn10() {
    // Popravite spodnjo vrstico
    let mut s = String::from("hello, ");

    helper3(&mut s);

    println!("Success!");
}
#[allow(unused_variables)]
fn helper3(s: &mut String) {}

/// ------------------------------------------------------------------------------------------------

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_stack() {
        on_stack();
    }

    #[test]
    fn test_swap_fn() {
        test_swap();
    }

    #[test]
    fn test_str_own() {
        str_own();
        str_own2();
    }

    #[test]
    fn test_wrong_and_helpers() {
        wrong();
        fn1();
        fn2();
        fn3();
        fn4();
        fn5();
        fn6();
        fn7();
    }

    #[test]
    fn test_fn8_fn9_fn10() {
        fn8();
        fn9();
        fn10();
    }
}
