// if1.rs
// 执行 `rustlings hint if1` 或在观察模式下使用 `hint` 子命令来获取提示。


pub fn bigger(a: i32, b: i32) -> i32 {
    if a>b{
        a
    }else{
        b
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
