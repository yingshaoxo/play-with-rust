use std::fs::File;

fn i_will_panic_for_sure(trigger: i32) -> Result<i32, String> {
    if trigger >= 0 {
        Ok(1)
    } else {
        Err(String::from(
            "shit, i'm an error. don't use panic in production since it's bad, it ends the program",
        ))
    }
}

fn i_will_panic() -> std::result::Result<std::fs::File, std::boxed::Box<(dyn std::error::Error)>> {
    let f = File::open("hello.txt"); //file does not exist
    match f {
        Ok(file) => Ok(file),
        Err(e) => Err(Box::new(e)),
    }
}

fn main() {
    match i_will_panic() {
        Ok(file) => println!("{:?}", file),
        Err(_error) => {
            // println!("{:?}", error)
            println!("{}", "hello, builtin error");
        }
    }

    match i_will_panic_for_sure(-1) {
        Ok(number) => println!("{:?}", number),
        Err(error) => println!("our own error: {:?}", error),
    }
}
