fn main() {
    let number = 3;

    //Rust는 boolean 타입이 아닌 것을 boolean 타입으로 자동 변환하지 않습니다.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }

    // if가 표현식이기 때문에 let 구문의 우측에 사용할 수 있다.
    // 만약 if블록이 정수형을 산출하는 식이고 else블록이 문자열을 산출하는 식이면 에러가 발생한다.
    // Rust에서는 변수가 가질 수 있는 타입이 오직 하나이다.
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    //loop는 Rust에게 그만두라고 명시하여 알려주기 전(break문)까지 코드 블럭을 반복 수행한다.
    loop{
        println!("again!");
        break;
    }

    //while 반복문
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");

    //while 배열 반복문
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    //for iter 반복문
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //for 반복문
    //for반복문이 안전하고 간결하기 때문에 가장 보편적으로 사용되는 반복문 구조자이다.
    //따라서 특정 획수만큼 코드를 반복하려는 경우에도, 대부분 for 반복문을 사용할 것이다.
    //Rust에서 기본 라이브러리로 제공하는 Range는 한숫자에서 다른 숫자 전까지 모든 숫자를 차례로 생성한다.
    //range를 역순으로 하려면 rev메소드를 사용하면 된다.

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    //섭씨 화씨 변환
    let value1 = toFahrenheit(100f64);
    let value2 = toCelsius(100f64);
    println!("100 celsius: {}F",value1);
    println!("100 fahrenheit: {}C", value2);
    //피보나치
    let value3 = fibonacci(5);
    println!("fibonacci 5step: {}", value3);

    //The Twelve Days of Christmas 가사
    printTheTwelveDaysofChristmas();
}

fn toFahrenheit(celsius : f64) -> f64{
    celsius * 1.8 + 32f64
}

fn toCelsius(fahrenheit : f64) -> f64{
    (fahrenheit - 32f64) / 1.8
}

fn fibonacci(step : i32) -> i32{
    let mut value = 1;
    let mut beforeValue = 1;
    for number in (1..(step+1)){
        let temp = value;
        value = value + beforeValue;
        beforeValue = temp;
    }
    value
}

fn printTheTwelveDaysofChristmas(){
    let lyrics = [
        "[Verse 1]", 
        "On the first day of Christmas my true love sent to me", 
        "A partridge in a pear tree", "", 
        "[Verse 2]", 
        "On the second day of Christmas my true love sent to me", 
        "Two turtle doves, and", 
        "A partridge in a pear tree", "", 
        "[Verse 3]",
        "On the third day of Christmas my true love sent to me", 
        "Three french hens", "Two turtle doves, and", 
        "A partridge in a pear tree", "", 
        "[Verse 4]", 
        "On the fourth day of Christmas my true love sent to me", 
        "Four calling birds", "Three french hens", "Two turtle doves, and", 
        "A partridge in a pear tree", "", 
        "[Verse 5]", 
        "On the fifth day of Christmas my true love sent to me", 
        "Five golden rings", "Four calling birds", "Three french hens", "Two turtle doves, and", 
        "A partridge in a pear tree"];
    for line in lyrics.iter(){
        println!("{}",line);
    }
}

