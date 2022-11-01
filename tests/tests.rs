use vecmac::vecmac;

#[test]
fn empty_vecmac() {
    let x: Vec<u32> = vecmac![];
    assert!(x.is_empty());
}

#[test]
fn single_vecmac() {
    let x: Vec<u32> = vecmac![36];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 36);
}

#[test]
fn double_vecmac() {
    let x: Vec<u32> = vecmac![36, 63];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 36);
    assert_eq!(x[1], 63);
}

#[test]
fn triple_vecmac() {
    let x: Vec<u32> = vecmac![36, 63, 96];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 3);
    assert_eq!(x[0], 36);
    assert_eq!(x[1], 63);
    assert_eq!(x[2], 96);
}

#[test]
fn multiple_vecmac() {
    let x: Vec<u32> = vecmac![36, 63, 96, 369];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 4);
    assert_eq!(x[0], 36);
    assert_eq!(x[1], 63);
    assert_eq!(x[2], 96);
    assert_eq!(x[3], 369);
}

#[test]
fn trailing_comma() {
    let x: Vec<u32> = vecmac![36, 63, 96,];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 3);
    assert_eq!(x[0], 36);
    assert_eq!(x[1], 63);
    assert_eq!(x[2], 96);
}

#[test]
fn clone_2() {
    let x: Vec<u32> = vecmac![36; 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 36);
    assert_eq!(x[1], 36);
}

#[test]
fn clone_2_non_literals() {
    let mut a = Some(36);
    let x: Vec<u32> = vecmac![a.take().unwrap(); 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 36);
    assert_eq!(x[1], 36);
}
