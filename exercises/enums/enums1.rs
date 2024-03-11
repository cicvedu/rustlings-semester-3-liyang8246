// enums1.rs
// 这次没有提示！ ;)

#[derive(Debug)]
enum Message {
    // TODO: 定义下面所用的消息类型
    Quit,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
