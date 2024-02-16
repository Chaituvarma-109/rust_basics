#[derive(Debug)]
#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    #[allow(dead_code)]
    fn inc_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    #[allow(dead_code)]
    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
}

#[allow(dead_code)]
fn change_username(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs() {
        let mut user1 = User {
            username: String::from("someusername123"),
            email: String::from("5HbQn@example.com"),
            active: true,
            sign_in_count: 1,
        };

        change_username(&mut user1, "johndoe");

        let mut user2 = User {
            email: String::from("5HbQn@example.com"),
            username: String::from("anotherusername567"),
            active: false,
            sign_in_count: 1,
        };

        user2.change_email("j5HbQn@example2.com");
        user2.inc_sign_in_count();

        dbg!(&user1);
        dbg!(&user2);
    }
}
