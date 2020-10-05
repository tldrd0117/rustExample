enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //... etc
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

//Option<T>를 이용하는 매칭
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    //match는 match내의 각 패턴을 통과하고, 해당 값에 "맞는" 첫 번째 패턴에서, 그 값은
    //실행 중에 사용될 연관된 코드 블록 안으로 떨어질 것입니다.
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("the values are {:?}, {:?}, {:?}", five, six, none);

    //_변경자(placeholder)
    //러스트는 또한 우리가 모든 가능한 값을 나열하고 싶지 않을 경우에 사용할 수 있는 패턴을 가지고 있습니다.
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => ()
    }
    //_ 패턴은 어떠한 값과도 매칭 될 것입니다.
    //match 표현식은 우리가 단 한 가지 경우에 대해 고려하는 상황에서는 다소 장황할 수 있습니다. 이러한 상황을 위하여,
    //러스트는 if let을 제공합니다.
}
