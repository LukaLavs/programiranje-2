#![allow(dead_code)]
// Napišite različne funkcije, ki kot argument sprejmejo zaprtje (closure) in ga pokličejo z različnimi argumenti, ki ustrezajo sledečim ocaml tipom:
// Tip zaprtja naj bo čim bolj splošen (Fn, FnOnce, FnMut).
//  apply_int: (int -> int) -> int -> int
//  apply: ('a -> 'b) -> 'a -> 'b
//  apply2: ('a -> 'a -> 'b) -> 'a -> 'a -> 'b
//  map: ('a -> 'b) -> 'a list -> 'b list  (Uporabite Vec<T> namesto list, predpostavite, funkcijo narediti najbolj splošno)
//  ponavljaj: int -> ('a -> 'a) -> 'a -> 'a // Ponovi funkcijo n-krat
//  filter: ('a -> bool) -> 'a list -> 'a list // Vrne seznam elementov, ki zadoščajo pogoju - uporabite Vec<T> namesto list in že vgrajeno funkcijo filter

fn apply_int<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn apply<A, B, F>(f: F, x: A) -> B
where
    F: Fn(A) -> B,
{
    f(x)
}

fn apply2<A, B, F>(f: F, x1: A, x2: A) -> B
where
    F: Fn(A, A) -> B,
{
    f(x1, x2)
}

fn map<A, B, F>(f: F, xs: &Vec<A>) -> Vec<B>
where
    F: Fn(&A) -> B,
{
    xs.into_iter().map(f).collect()
}

fn ponavljaj<A, F>(n: usize, f: F, x: A) -> A
where
    F: Fn(A) -> A,
{
    (0..n).fold(x, |acc, _| f(acc))
}

fn filter<A, F>(f: F, xs: &Vec<A>) -> Vec<A>
where
    F: Fn(&A) -> bool,
    A: Clone,
{
    xs.into_iter().filter(|x| f(x)).cloned().collect()
}
// Vzemite zaporedja iz prejšnjih vaj in naredite nov objekt, ki sprejme zaporedje in ga naredi iterabilnega

// pass 

// Iteratorji

// Napišite funkcijo, ki sprejme vektor XYZ in s pomočjo iteratorja naredi W
// števil in izpiše vsako v svojo vrstico
// nizov in izpiše njihove dolžine
// nizov in vrne vsoto njihovih dolžin
// vektor parov (i32, i32) in vrne vsoto njihovih pozitivnih produktov
// dva vektorja <i32> in vrne vektor, ki vsebuje vsote parov
// dva vektorja <i32> in vrne vsoto poparjenih pozitivni produktov
// vektor Option<T> in izpiše vse T-je
// vektor Option<T> in vrne število Some-ov
// odfiltrira števila deljiva s 3

fn print_xyz(v: &Vec<i32>) {
    v.into_iter().for_each(|x| println!("{}", x));  
}
fn print_lengths(v: &Vec<String>) {
    v.into_iter().for_each(|s| println!("{}", s.len()));
}
fn sum_lengths(v: &Vec<String>) -> usize {
    v.into_iter().map(|s| s.len()).sum()
}
fn sum_products(v: &Vec<(i32, i32)>) -> i32 {
    v.into_iter()
        .map(|(x, y)| x * y) 
        .filter(|&prod| prod > 0)
        .sum()
}
fn sum_pairs(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    v2.iter().zip(v1.iter()).map(|(x, y)| x + y).collect()
}
fn sum_positive_products(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    v1.iter().zip(v2.iter()).map(|(x, y)| x * y).filter(|&prod| prod > 0).collect()
}


// Dopolnite spodnjo funkcijo, da vrne niz, kjer so vse prve črke posameznih besed velike
// ["Just,", " ", "hello", " ", "world", "!"] -> "Just, Hello World", "!"
// pub fn capitalize_words_string(words: &[&str]) -> String {
//     panic!("Not implemented");
// }
// Napišite funkcijo `fakulteta`, ki izračuna fakulteto števila n. Uporabite iteratorje (torej brez lastne for zanke, rekurzije)
// Namig: fold, reduce, `..`...

// -------------------------------------------------------------------------------------------------
// Dodatno:
// Koda vzeta iz googlvih rust vaj:
// Vse se da lepo narediti samo z iteratorji (brez indeksov, brez for zank, brez mutabilnosti)
/*
/// Calculate the differences between elements of `values` offset by `offset`,
/// wrapping around from the end of `values` to the beginning.
///
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
fn offset_differences<N>(offset: usize, values: Vec<N>) -> Vec<N>
where
    N: Copy + std::ops::Sub<Output = N>,
{
    unimplemented!()
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_custom_type() {
    assert_eq!(
        offset_differences(1, vec![1.0, 11.0, 5.0, 0.0]),
        vec![10.0, -6.0, -5.0, 1.0]
    );
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}



*/

fn main() {
    println!("=== apply_int ===");
    let r = apply_int(|x| x * 2, 5);
    println!("apply_int(|x| x*2, 5) = {}", r);

    println!("\n=== apply ===");
    let r = apply(|s: &str| s.len(), "hello");
    println!("apply(|s| s.len(), \"hello\") = {}", r);

    println!("\n=== apply2 ===");
    let r = apply2(|a, b| a + b, 3, 4);
    println!("apply2(|a,b| a+b, 3, 4) = {}", r);

    println!("\n=== map ===");
    let v = vec![1, 2, 3, 4];
    let mapped = map(|x| x * x, &v);
    println!("original: {:?}", v);
    println!("squared:  {:?}", mapped);

    println!("\n=== ponavljaj ===");
    let r = ponavljaj(5, |x| x + 1, 0);
    println!("ponavljaj(5, |x| x+1, 0) = {}", r);

    println!("\n=== filter ===");
    let v = vec![1, -2, 3, -4, 5];
    let filtered = filter(|x| *x > 0, &v);
    println!("original: {:?}", v);
    println!("positive only: {:?}", filtered);

    println!("\n=== print_xyz ===");
    let v = vec![10, 20, 30];
    print_xyz(&v);

    println!("\n=== print_lengths ===");
    let words = vec!["hello".into(), "rust".into(), "world".into()];
    print_lengths(&words);

    println!("\n=== sum_lengths ===");
    let sum = sum_lengths(&words);
    println!("sum of lengths = {}", sum);

    println!("\n=== sum_products ===");
    let pairs = vec![(1, 2), (-3, 4), (5, -6), (2, 3)];
    let sum = sum_products(&pairs);
    println!("sum of positive products = {}", sum);

    println!("\n=== sum_pairs ===");
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let sums = sum_pairs(&a, &b);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("pairwise sums = {:?}", sums);

    println!("\n=== sum_positive_products ===");
    let prod = sum_positive_products(&a, &b);
    println!("positive products = {:?}", prod);

    println!("\n=== done ===");
}
