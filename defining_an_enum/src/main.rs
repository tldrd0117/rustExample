enum IpAddrkind {
    V4,
    V6,
}

struct IpAddrStruct{
    kind: IpAddrkind,
    address: String,
}

enum IpAddr {
    V4(String),
    V6(String)
}

enum IpAddrSplit{
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let home = IpAddrStruct{
        kind: IpAddrkind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddrStruct{
        kind: IpAddrkind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddrSplit::V4(127, 0, 0, 1);
    let loopback = IpAddrSplit::V6(String::from("::1"));

    //IP 주소와 그 종류를 저장하는 것은 흔하기 떄문에, 표준 라이브러리에 사용할 수 있는 정의가 있다.

    //struct와 enum이 다른점은 모든 variants가 하나의 타입으로 그룹화 된다는 것이다.

    //Rust는 null이 없지만 값의 존재 혹은 부재의 개념을 표현할 수 있는 열거형이 있습니다.
    //이 열거형의 Option<T> 이며 표준 라이브러리에 정의가 았습니다.
}
