//소유권 규칙
//1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
//2. 한번에 딱 하나의 오너만 존재할 수 있다.
//3. 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다.(dropped).
fn main() {
    scope();
    stringType();
    copy();
    moveFn();
    clone();
    ownershipAndFunction();
    returnAndScope();
    need_to_be_passed_back_and_use_again();
}

fn scope(){
                        //s는 유효하지 않는다. 아직 선언이 안됐음.
    let s = "hello";    //s는 이 지점부터 유효합니다.
                        //s흫 가지고 뭔가 합니다.
                        // 이스코프틑 이제 끝이므로 s는 더이상 유효하지 않습니다.
}

//스트링 리터럴의 경우, 텍스트가 최종 실행파일에 직접 하드코딩 되고, 이렇게 하면 스트링 리터럴이 빠르고 효율적이 됩니다.
//불행하게도, 컴파일 타임에 크기를 알 수 없는 경우 및 실행 중 크기가 변할 수도 있는 경우는 텍스트 조각을 바이너리 파일에 접어넣을 수 없습니다.
//String 타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어졌고, 우리는 힙에서 컴파일 타임에는 알 수 없는 어느 정도
//크기의 메모리 공간을 할당받아 내용물을 저장할 필요가 있습니다.
//1. 런타임에 운영체제로부터 메모리가 요청되어야 한다.
//2. String의 사용이 끝났을 떄 운영체제에게 메모리를 반납할 방법이 필요하다.
//첫번째는 우리가 직접 수행합니다. 우리가 String::from을 호출하면, 구현부분에서 필요한 만큼의 메모리를 요청합니다.
//두번쨰는 가비지콜렉터(GC)를 갖고 있는 언어들의 경우, GC가 더이상 사용하지 않는 메모리 조각을 계속해서 찾고 지워주며,
//우리는 프로그래머로서 이와 관련한 생각을 안해도 됩니다.
//GC가 없을 경우, 할당받은 메모리가 더 필요없는 시점을 알아서 명시적으로 이를 반납하는 코드를 호출하는 것은 프로그래머의
//책임입니다.
//러스트는 다른 방식으로 이 문제를 다룹니다. 메모리는 변수가 소속되어 있는 스코프 밖으로 벗어나는 순간 자동으로 반납됩니다.
fn stringType(){
    let s = String::from("hello");
    let mut s = String::from("hello");
    s.push_str(", world"); //push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.
    println!("{}", s);
}

//스택에만 있는 데이터: 복사
fn copy(){
    let x = 5;
    let y = x;
    // 이경우 고정된 크기의 단순한 값이기 떄문에 5라는 값들은 스택에 푸쉬됩니다.
    // 정수형과 같이 커뮤ㅏ일 타임에 결정되어 있는 크기의 타입은 스택에 모두 저장되기 떄문에, 실제 값의
    // 복사본이 빠르게 만들어질 수 있습니다. 이는 변수 y가 생성된 후에 x가 더 이상 유효하지 않도록 해야할 이유가
    // 없다는 뜻입니다.
    println!("x = {}, y = {}", x, y);

}

//변수와 데이터가 상호작용하는 방법: 이동(move)
fn moveFn(){

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);
    // 이경우 러스트는 얕은 복사와 깊은복사와 같은 개념으로 동작하지 않고
    // 첫번째 변수(s1)를 무효화 하고 두번 째 변수(s2)가 hello를 가리키게 된다.
    // 이를 얕은 복사라고 부르든 대신 move라고 말합니다. 

}

//변수와 데이터가 상호작용하는 방법: 클론(clone)
fn clone(){
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    println!("s1 = {}, s2 = {}", s1, s2);
    // 깊은 복사와 같은 개념으로 동작한다.
}

// 소유권과 함수
fn ownershipAndFunction(){
    let s = String::from("hello");  // s가 스코프 안으로 들어왔습니다.
    
    takes_ownership(s);             // s의 값이 함수 안으로 이동했습니다.
                                    // 그리고 이제 더이상 유효하지 않습니다.
    let x = 5;                      // x가 스코프 안으로 들어왔습니다.

    makes_copy(x);                  // x가 함수 안으로 이동했습니다만,
                                    // i32는 Copy가 되므로, x를 이후에 계속
                                    // 사용해도 됩니다.
}   //여기서 x는 스코프 밖으로 나가고, s도 그 후 나갑니다. 하지만 s는 이미 이동되었으므로,
    //별다른 일이 발생하지 않습니다.

fn takes_ownership(some_string: String){   //some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
}   //여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
    //해제되었습니다.

fn makes_copy(some_integer: i32){   //some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
}   //여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.

// 반환값과 스코프
fn returnAndScope(){
    let s1 = gives_ownership();     //gives_ownership은 반환값을 s1에게 이동시킵니다.

    let s2 = String::from("hello"); //s2가 스코프 안에 들어왔습니다.

    let s3 = takes_and_gives_back(s2);  //s2는 takes_and_gives_back 안으로 이동되었고,
                                        //이 함수가 반환값을 s3으로도 이동시켰습니다.
}
// 여기서 s3는 스코프 밖으로 벗어났으며 drop이 호출됩니다. s2는 스코프 밖으로 벗어났지만
// 이동되었으므로 아무 일도 일어나지 않습니다.
// s1은 스코프 밖으로 벗어나서 drop이 호출됩니다.

fn gives_ownership() -> String{
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn need_to_be_passed_back_and_use_again(){
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    //이건 너무 많이 나간 의례절차고 일반적인 개념로서는 과한 작업이 됩니다.
    //러스트는 이를 위한 기능을 갖고 있으며, 참조자라고 부릅니다.

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}