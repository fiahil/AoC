use d5::computer::Computer;

#[test]
fn test_0() {
    assert_eq!(Computer::new("1,0,0,0,99").run().raw_mem(), "2,0,0,0,99");
}

#[test]
fn test_1() {
    assert_eq!(Computer::new("2,3,0,3,99").run().raw_mem(), "2,3,0,6,99");
}

#[test]
fn test_2() {
    assert_eq!(Computer::new("2,4,4,5,99,0").run().raw_mem(), "2,4,4,5,99,9801");
}

#[test]
fn test_3() {
    assert_eq!(
        Computer::new("1,1,1,4,99,5,6,0,99").run().raw_mem(),
        "30,1,1,4,2,5,6,0,99"
    );
}

#[test]
fn test_mode() {
    assert_eq!(Computer::new("1002,4,3,4,33").run().raw_mem(), "1002,4,3,4,99");
}

#[test]
fn test_neg() {
    assert_eq!(Computer::new("1101,100,-1,4,0").run().raw_mem(), "1101,100,-1,4,99");
}
