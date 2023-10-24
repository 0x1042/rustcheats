use std::fmt::Display;

fn print_slice<T>(arg: &[T])
where
    T: Display,
{
    for x in arg {
        println!("x {}", x)
    }
}

#[cfg(test)]
mod tests {
    use crate::examples::types::print_slice;

    #[test]
    fn test_types() {
        println!("128 is {}->{}", u128::MIN, u128::MAX);
        println!("64 is {}->{}", u64::MIN, u64::MAX);
        println!("32 is {}->{}", u32::MIN, u32::MAX);
        println!("16 is {}->{}", u16::MIN, u16::MAX);
        println!("8 is {}->{}", u8::MIN, u8::MAX);

        let port = 10086u16;

        // 二进制 等于十进制的4
        let num1 = 0b100;
        // 八进制
        let num2 = 0o23;
        // 十六进制
        let num3 = 0x1010;

        println!("num1 {} {}", num2, num3);

        let nums: [usize; 4] = [1, 2, 3, 4];
        // 初始化时给定默认值
        let mut buf = [1u8; 2];

        print_slice(buf.as_slice());
        let nums2 = vec![5, 6, 7, 8];
        let nums3: &Vec<i32> = &nums2;
        print_slice(nums3);

        let hello = "hello world!".to_owned();

        let app_supprot = r"~/Library/Application Support";
        let app_supprot1 = r###"~/Library/Application Support/"###;

        let method: &[u8; 3] = b"GET";

        let t1 = ("hello", 1024);
        let t2 = ("world", 3.14, 2048);

        // tuple 只能通过常量下标引用
        println!("t2.0->{}", t2.0);
        println!("t2.1->{}", t2.1);
        println!("t2.2->{}", t2.2);

        let (first, second) = ("hello", "world");
        println!("{first}->{second}");

        let noodles = "noodles".to_owned();

        println!("len:{} cap:{}", noodles.len(), noodles.capacity());
    }
}
