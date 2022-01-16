#[derive(PartialEq, Debug)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber { 0: value })
        } else {
            Err(())
        }
    }
}

#[test]
fn test_tryfrom() {
    assert_eq!(EvenNumber::try_from(2), Ok(EvenNumber(2)));
    assert_eq!(EvenNumber::try_from(3), Err(()));

    let a: Result<EvenNumber, ()> = 2.try_into();
    assert_eq!(a, Ok(EvenNumber(2)));
    let b: Result<EvenNumber, ()> = 3.try_into();
    assert_eq!(b, Err(()));
}
