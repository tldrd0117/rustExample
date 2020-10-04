fn main() {
    let s1 = String::from("hello");
    let len = calcalate_length(&s1);
    //&s1문법은 우리가 s1의 값을 참조하지만 소유하지는 않는 참조자를 생성하도록 해줍니다.
    //소유권을 갖고 있지는 않기 때문에, 이 참자조가 가리키는 값은 참조자가 스코프 밖으로 벗어났을 때도 메모리가
    //반납되지 않을 것입니다.

    println!("The length of '{}' is {}.", s1, len);

    //가변참조자(Mutable References)
    //가변참조자는 딱 한가지 큰 제한이 있습니다. 특정한 스코프 내에 특정한 데이터 조각에 대한 가변 참조자를
    //딱 하나만 만들 수 있다는 겁니다.
    let mut s = String::from("hello");
    change(&mut s);

    println!("{}!",s);

    //여러개의 가변 참조자를 만드는 방법
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    //댕글링 참조자(Dangling References)
    //포인터가 있는 언어에서는 자칫 잘못하면 댕글링 포인터를 만들기 쉬운데,
    //댕글링 포인터란 어떤 메모리를 가리키는 포인터를 보존하는 동안, 그 메모리를 해제함으로써 다른 개체에서 사용하도록
    //줘버렸을 지도 모를 메모리를 참조하고 있는 포인터를 말합니다. 이와는 반대로, 러스트에서는 컴파일러가
    //모든 참조자들이 댕글링 참조자가 되지 않도록 보장해 줍니다.
    //컴파일러는 그 참조자가 스코프 밖으로 벗어나기전에는 데이터가 스코프 밖으로 벗어나지 않을 것임을 확인해 줄 것 입니다.
    //dangle()
    no_dangle();

    //참조자의 규칙
    //1. 어떠한 경우이든 간에, 여러분은 아래 둘 다는 아니고 둘 중 하나만 가질 수 있습니다
    //- 하나의 가변 참조자
    //- 임의 개수의 불변 참조자들
    //2. 참조자는 항상 유효해야만 한다.
}
//실제 값 대신 참조자를 파라미터로 갖고 있는 함수는 소유권을 갖고 있지 않기 때문에 소유권을 되돌려주기 위해
//값을 다시 반환할 필요도 없다는 뜻이 됩니다.
//함수의 참조자를 만드는 것을 빌림(borrowing)이라고 부릅니다.
//빌린 값을 고치려하면 참조자는 불변이기 때문에 참조하는 어떤 것을 변경하는 것은 허용되지 않습니다.
fn calcalate_length(s: &String) -> usize{
    s.len()
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

// fn dangle() -> &String{ //dangle은 String의 참조자를 반환합니다.
//     let s = String::from("hello"); // s는 새로운 String입니다.
//     &s //우리는 String s의 참조자를 반환합니다.
// } //여기서 s는 스코프를 벗어나고 버려집니다. s의 메모리는 사라집니다.
// //위험합니다.

fn no_dangle() -> String{
    let s = String::from("hello");
    s
}