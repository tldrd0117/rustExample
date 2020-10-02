fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    //let guess: u32 = "42".parse().expect("Not a number!");
    //Rust는 타입이 고정된 언어이다. String을 parse를 사용하여 숫자로 변환했던 경우처럼 타입의 선택 폭이 넓은 경우는
    //반드시 타입의 명시를 첨가해야 한다.
    //
    // 정수형 타입
    // Length   Signed  Unsigned
    // 8 bit    i8      u8
    // 16bit    i16     u16
    // 32bit    i32     u32
    // 64bit    i64     u64
    // arch     isize   usize
    // 각 부호 변수는  -(2^(n - 1)) 부터 2^(n - 1) - 1 까지의 값을 포괄합니다. 여기서 n은 사용되는 타입의 비트 수 입니다.
    // 추가로 isize와 usize 타입은 당신의 프로그램이 동작하는 컴퓨터 환경이 64비트인지 아닌지에 따라 결정됩니다.
    // byte리터럴을 제외하고 모든 정수형 리터럴은 57u8과 같은 타입 접미사와 1_000과 같이 시각적인 구분을 위한 _의 사용을 허용합니다.
    // 확실하게 정해진 경우가 아니면 Rust의 기본 값인 i32가 일반적으로는 좋은 선택입니다. 이는 일반적으로 가장 빠르기 때문이죠.
    // 심지어 64비트 시스템에서도요. 
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("The Value is {} {} {} {} {}", decimal, hex, octal, binary, byte);

    // 부동소수점 타입
    // Rust의 부동소수점 타입은 f32와 f64로 예상하신 대로 각기 32bit와 64bit의 크기를 갖습니다.
    // 기본 타입은 f64인데, 그 이유는 최신의 CPU 상에서는 f64가 f32와 대략 비슷한 속도를 내면서도 더 정밀한 표현이 가능하기 때문입니다.

    let x = 2.0;        // f64
    let y: f32 = 3.9;   // f32
    println!("The value is {} {}", x, y);

    // 수학적 연산들.
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.3 - 4.3;

    // multiplication
    let product = 4* 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("The value is {} {} {} {} {}", sum, difference, product, quotient, remainder);

    //boolean 타입
    let t = true;
    let f: bool = false;

    println!("The value is {} {}", t, f);

    //char 타입
    // Rust의 char는 작은 따옴표로 쓰고 스트링은 큰따옴표를 쓴다.
    //char 타입은 unicode scalar를 표한하는 값이고 이는 ASCII 보다 많은 표현을 가능하게 합니다.
    //값의 범위는 U+0000에서 U+D7FF 그리고 U+E000에서 U+10FFFF를 포괄한다.
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value is {} {} {}", c, z, heart_eyed_cat);

    // 복합 타입들
    // 튜플은 다양한 타입의 몇 개의 숫자를 집합시켜 하나의 복합 타입으로 만드는 일반적인 방법입니다.
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    //구조 헤체
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // .뒤에 우리가 접근하길 원하는 값의 색인을 넣는 것을 통해 튜플의 요소에 직접적으로 접근할 수 있습니다.
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    println!("The value of y is: {} {} {}", five_hundred, six_point_four, one);

    // 배열
    // 튜플과는 다르게, 배열의 모든 요소는 모두 같은 타입이여야 합니다.
    // 배열이 유용할 떄는 당신의 데이터를 heap보다 stack에 할당하는 것을 원하거나, 항상 고정된 숫자의 요소를 갖는다는 확신이 들 떄 입니다.
    // 이들은 벡터 타입처럼 가변적이지 않습니다... 벡터 타입은 유사 집합체로 표준 라이브러리에서 제공되며 확장 혹은 축소가 가능합니다.
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    println!("The value of first and second are: {} {} ", first, second);

    //index를 이용하여 요소에 접근하려고 하면 Rust는 지정한 색인이 배열 길이보다 작은지 확인합니다. 배열 길이보다 길면
    // Rust는 프로그램이 오류와 함께 종료 될 떄 Panic 합니다.

}
