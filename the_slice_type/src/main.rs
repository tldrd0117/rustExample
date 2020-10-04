fn main() {
    //스트링을 입력 받아 그 스트링에서 찾은 첫번째 단어를 반환하는 함수를 작성하세요.
    //만일 함수가 공백문자를 찾지 못한다면, 이는 전체 스트링이 한 단어라는 의미이고, 전체 스트링이 반환되어야 합니다.
    let mut s = String::from("hello world");

    let word = first_word(&s); //word는 5를 갖게 될 것입니다.

    s.clear(); // 이 코드는 String을 비워서 ""로 만들게 됩니다.

    // word는 여기서 여전히 5를 갖고 있지만, 5라는 값을 의미있기 쓸 수 있는 스트링은 이제 없습니다.
    // word는 이제 완전 유효하지 않습니다!

    //스트링 슬라이스
    let s = String::from("hello world");
    //s는 hello world의 h의 포인터를 가지고 있고 len은 11이다 
    let hello = &s[0..5];
    let world = &s[6..11];
    //world는 hello world의 w의 포인터를 가지고 있고 len은 5이다.
    let len = s.len();
    //전체 스트링의 슬라이스
    let slice = &s[0..len];
    let slice = &s[..];

    let word = first_word_slice(&s);
    //s.clear()을 사용하면 Error 발생
    //first_word는 스트링을 이후에 비워버려서 인덱스가 유효하지 않는 상태가 된다.
    //슬라이스를 사용하면 이러한 버그를 불가능하게 하며 컴파일 타임 오류를 발생시킨다.
    println!("the first word is: {}", word);

    let my_string = String::from("hello world");

    //first_word가 `String`의 슬라이스로 동작합니다.
    let word = first_word_slice_fix(&my_string[..]);

    let my_string_literal = "hello world";

    //first word가 스트링 리터럴의 슬라이스로 동작합니다.

    let word = first_word_slice_fix(&my_string_literal[..]);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작합니다!
    let word = first_word_slice_fix(my_string_literal);

    //그 밖의 슬라이스들

    //배열의 일부를 참조
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    //String은 as_bytes 메소드를 이용하여 바이트 배열로 변환

    for(i, &item) in bytes.iter().enumerate() {
        //&item은 튜플 내의 한 바이트에 대응하는 패턴을 기술한 것
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

//스트링 슬라이스를 나타내는 타입은 &str이다.
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

//함수가 String의 참조자 대신 스트링 슬라이스를 갖도록 정의하는 것은 어떠한 기능적인
//손실 없이도 더 일반적으로 유용하게 해줍니다.
fn first_word_slice_fix(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> (usize, usize) {
    (1,1)
}