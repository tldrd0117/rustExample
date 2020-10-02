fn main() {
    another_function(3);

    //구문과 표현식
    //let x = (let y = 6);는 작동하지 않는다.
    // x = y = 6;은 작동하지 않는다.
    //표현식은 구문의 부분일 수 있습니다.
    let y = 6;//이란 구문을 갖는데 6은 6이란 값을 산출하는 표현식이다.
    //매크로를 호출하는 것도 표현식이다.
    //새로운 범위를 생성하는데 사용하는 block{}도 표현식이다
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    // 해당 block{}은 4를 산출합니다. 이 값은 let 구문의 일부로 y에 bound 됩니다.
    println!("The value of y is: {}", y);

    //함수는 그들을 호출한 코드에 값을 반환할 수 있습니다. 우리는 반환되는 값을 명명해야 할 필요는 없지만, 그들의 타입은
    //화살표(->) 뒤에 선언해야합니다. Rust에서 반환 값은 함수 본문의 마지막 표현식의 값과 동일합니다.
    //return 키워드와 값을 써서 함수로부터 일찍 반환할 수 있지만, 대부분의 함수들은 암묵적으로 마지막 표현식을 반환합니다.
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

}

//Rust 코드는 snake shape(aaa_bbb 형태)를 변수나 함수 이름의 형식 규칙으로 사용합니다.
//Rust는 당신의 함수의 위치를 신경쓰지 않습니다. 어디든 정의만 되어 있으면 됩니다.
//함수의 정의에 타입을 명시하여 코드 내 다른 부분에서 이들을 사용하는 것을 통해 당신의 의도를 추측하지 않아도 되게 됩니다.
//함수의 선언부에서, 여러분은 반드시 각 매개변수의 타입을 정의해야 합니다.
fn another_function(x: i32) {
    println!("Another function. {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}