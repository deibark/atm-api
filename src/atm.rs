pub fn withdraw(mut amount: u32) -> Result<Vec<u32>, &'static str> {

    let notes = vec![100, 50, 20, 10];
    let mut result = vec![];

    if amount % 10 != 0 {
        return Err("NoteUnavailableException");
    }

    for i in 0..4 {
        if amount >= notes[i] {
            let note_cnt = amount/ notes[i];

            for _ in 0..note_cnt {
                result.push(notes[i]);
            }

            amount = amount - note_cnt * notes[i];
        }
    }

    Ok(result)

}

#[test]
fn withdraw_zero() {
    let result = withdraw(0).ok();
    let expect = vec![];

    assert_eq!(result, Some(expect));
}

#[test]
fn withdraw_125() {
    let result = withdraw(125).err();
    let expect = "NoteUnavailableException";

    assert_eq!(result, Some(expect));
}

#[test]
fn withdraw_10() {
    let result = withdraw(10).ok();
    let expect = vec![10];

    assert_eq!(result, Some(expect));
}

#[test]
fn withdraw_30() {
    let result = withdraw(30).ok();
    let expect = vec![20, 10];

    assert_eq!(result, Some(expect));
}

#[test]
fn withdraw_40() {
    let result = withdraw(40).ok();
    let expect = vec![20, 20];

    assert_eq!(result, Some(expect));
}

#[test]
fn withdraw_80() {
    let result = withdraw(80).ok();
    let expect = vec![50, 20, 10];

    assert_eq!(result, Some(expect));
}

#[test]
fn withdraw_180() {
    let result = withdraw(180).ok();
    let expect = vec![100, 50, 20, 10];

    assert_eq!(result, Some(expect));
}

#[test]
fn withdraw_220() {
    let result = withdraw(220).ok();
    let expect = vec![100, 100, 20];

    assert_eq!(result, Some(expect));
}

#[test]
fn withdraw_360() {
    let result = withdraw(360).ok();
    let expect = vec![100, 100, 100, 50, 10];

    assert_eq!(result, Some(expect));
}
