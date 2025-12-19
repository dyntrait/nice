/// 一个通用的工具函数示例
pub fn hello() -> String {
    "Hello from common-utils!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(hello(), "Hello from common-utils!");
    }
}
