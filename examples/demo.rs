use reusable::{reusable, reuse};

#[reusable(test_name)]
#[derive(Debug)]
pub struct Name {
    pub firstname: String,
    pub surname: String,
}

#[reusable(test_middlename)]
#[derive(Debug)]
pub struct Middlename {
    pub middlename: String,
}

#[reuse(test_name, test_middlename)]
#[derive(Debug)]
pub struct Fullname {
    pub nickname: String,
}

fn main() {
    let example = Fullname {
        firstname: "Bob".to_string(),
        middlename: "Frank".to_string(),
        surname: "Junior".to_string(),
        nickname: "John".to_string(),
    };
    dbg!(example);
}
