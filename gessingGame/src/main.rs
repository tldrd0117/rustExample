extern crate rand;
//extern crate rand;을 추가하여 러스트에게 우리가 외부에 의존하는 크레이트가 있음을 알립니다.

use std::io;
//사용자 입력을 받고 결과값을 표시하기 위해서는 io(input/output) 라이브러리를 스코프로 가져와야 한다.
//io라이브러리는 std라고 불리는 표준 라이브러리에 있다.
//러스트는 모든 프로그램의 스코프에 prelude 내의 타입들을 가져온다.
//원하는 타입이 prelude에 없다면 use문을 활용하여 명시적으로 그 타입을 가져와야한다.
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    //println!은 string을 화면에 표시하는 매크로

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //rand::thread_rng 함수는 OS가 시드(seed)를 정하고 현재 스레드에서만 사용되는 특별한 정수생성기를 돌려 줍니다. 
    //다음으로 우리는 get_range 메소드를 호출합니다. 이 메소드는 Rng trait에 정의되어 있으므로 
    //use rand::Rng 문을 통해 스코프로 가져올 수 있습니다
    //gen_range 메소드는 두 개의 숫자를 인자로 받고 두 숫자 사이에 있는 임의의 숫자를 생성합니다. 1에서 100

    println!("The secret number is {}", secret_number);

    loop {
        
        println!("Please input your guess.");

        let mut guess = String::new();
        //러스트에서 변수는 기본적으로 불변(immutable)이다.
        //mut를 이용하여 가변변수로 만들 수 있다.
        //let mut guess 가 guess라는 이름의 가변변수임을 알 수 있다.
        //String::new의 결과값인 새로운 String 인스턴스가 묶이는 대상이 된다.
        //String은 표준 라이브러릴에서 제공하는 확장 가능한(growable) UTF-8 인코딩의 문자열 타입이다.
        //::에 있는 ::는 new가 String 타입의 연관함수 임을 나타낸다.
        //연관함수는 하나의 타입을 위한 함수이며 이경우에는 하나의 String 인스턴스가 아니라 String 타입을 위한 함수이다.
        //몇몇 언어에서는 이것을 정적 메소드라 부른다.
        //요약하자면 let mut guess = String::new();는 새로운 빈 String 인스턴스와 연결된 가변변수를 생성한다.

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        //use std::io;를 이용하여 표준 라이브러리의 input/output 기능을 포함한 것을 떠올려보면
        //이제 우리는 io의 연관함수인 stdin을 호출한다.
        //만약 use std::io가 없다면 함수 호출 시 std::io::stdin처럼 작성해야한다
        //stdin함수는 표준 입력의 핸들(handle)의 타입인 std::io::Stdin의 인스턴스를 돌려준다.
        //read_line은 사용자가 표준입력에 입력할 때마다 입력된 문자들을 하나의 문자열에 저장하므로 인자로 값을 저장할 문자열이 필요합니다.
        //그 문자열 인자는 사용자 입력을 추가하면서 변경되므로 가변이어야 합니다.
        //&는 코드의 여러 부분에서 데이터를 여러 번 메모리에 복사하지 않고 접근하기 위한 방법을 제공하는 참조자 임을 나타냅니다.
        //참조자는 복잡한 특성으로서 러스트의 큰 이점 중 하나가 참조자를 사용함으로써 얻는 안전성과 용이성입니다.
        //지금 당장은 참조자가 변수처럼 기본적으로 불변임을 알기만 하면 됩니다. 따라서 가변으로 바꾸기 위해 &guess가 아니라 &mut guess로 작성해야 합니다.
        //read_line에서 돌려주는 값은 io::Result입니다. 
        //Result 타입은 열거형로써 enums 라고 부르기도 합니다. 열거형은 정해진 값들만을 가질 수 있으며
        //이러한 값들은 열거형의 variants라고 부릅니다.
        //Result의 variants는 Ok와 Err입니다.
        //io::Result 인스턴스는 expect 메소드을 가지고 있습니다.
        //만약 io::Result 인스턴스가 Err 일 경우 exprect 메소드는 프로그램이 작동을 멈추게 하고
        //expect에 인자로 넘겼던 메세지를 출력하도록 합니다.
        //만약 Ok값이라면 expect는 Ok가 가지고 있는 결과값을 돌려주어 사용할 수 있도록 합니다.
        //이 경우 결과값은 사용자가 표준 입력으로 입력했던 바이트의 개수입니다.
        //만약 expect를 호출하지 않는다면 컴파일은 되지만 경고가 나타납니다.

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        //i32는 32비트 정수, u32는 32비트의 부호없는 정수, i64는 64비트의 정수이다.
        //문자열의 parse 메소드는 문자열을 숫자형으로 파싱합니다.
        //guess 뒤의 콜론(:)은 변수의 타입을 명시했음을 의미합니다.

        println!("You guessed: {}", guess);
        //{}는 변자로써 값이 표시되는 위치를 나타냅니다.

        match guess.cmp(&secret_number){
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            },
        }
        // match 표현문을 이용하여 cmp가 guess와 secret_number를 비교한 결과인 
        // Ordering의 값에 따라 무엇을 할 것인지 결정할 수 있습니다.
        // match 표현식은 arm 으로 이루어져 있습니다. 하나의 arm은 하나의 패턴 과 
        // match 표현식에서 주어진 값이 패턴과 맞는다면 실행할 코드로 이루어져 있습니다.

    }   
}
