
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    //하나의 패턴만 매칭 시키는 경우에는 _ => () 너무 많은 보일러 플레이트 코드입니다.

    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("no same");
    }
}
