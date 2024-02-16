#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

#[allow(dead_code)]
fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::ChangeColor(r, g, b) => println!("ChangeColor: {} {} {}", r, g, b),
        Message::Move { x, y } => println!("Move: {} {}", x, y),
        Message::Write(s) => println!("Write: {}", s),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_large_enum() {
        let msg = Message::Write(String::from("hello"));
        process_message(msg);

        let msg = Message::ChangeColor(10, 20, 30);
        process_message(msg);

        let msg = Message::Move { x: 10, y: 20 };
        process_message(msg);

        let msg = Message::Quit;
        process_message(msg);
    }

    #[test]
    fn test_match_literals() {
        let number = 40;

        let res: &str = match number {
            1 => "1",
            2 => "2",
            20 => "20",
            _ => "Not 20",
        };

        println!("res: {}", res);
    }

    #[test]
    fn test_match_option() {
        let some_num: Option<i32> = Some(10);
        // let prob_none: Option<i32> = None;

        let my_int = if let Some(num) = some_num {
            // println!("num: {}", num);
            num
        } else {
            panic!("some_num is None");
        };

        println!("my_int: {}", my_int);

        // let res: i32 = match some_num {
        //     Some(num) => num,
        //     None => panic!("some_num is None"),
        // };

        // println!("res: {}", res);
    }

    #[test]
    fn test_match_result() {
        let some_res: Result<i32, String> = Ok(50);
        // let some_err: Result<i32, String> = Err("there was a problem".to_string());

        match &some_res {
            Ok(res) => println!("res: {}", res),
            Err(err) => println!("err: {}", err),
        };

        let res = if let Ok(r) = some_res {
            r
        } else {
            panic!("some_res is Err");
        };

        println!("res: {}", res);
    }

    #[test]
    fn test_match_guard() {
        let pair: (i32, i32) = (2, 5);

        match pair {
            (x, y) if x == y => println!("equal"),
            (x, y) if x + y == 0 => println!("zero"),
            _ => println!("not equal"),
        }
    }

    #[test]
    fn test_match_struct() {
        struct Location {
            x: i32,
            y: i32,
        }

        let loc = Location { x: 10, y: 20 };

        match loc {
            Location { x, y: 0 } => println!("x: {}", x),
            Location { x: 0, y } => println!("y: {}", y),
            Location { x, y } => println!("x: {}, y: {}", x, y),
        };
    }
}
