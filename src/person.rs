use bcrypt::{hash, verify, DEFAULT_COST};

struct Person {
    fist_name: String,
    last_name: String,
    gender: String,
    age: u8
}

impl Person {
    fn new (first: &str, last: &str, gender: &str, age: u8) -> Person {
        Person {
            fist_name: first.to_string(),
            last_name: last.to_string(),
            gender: gender.to_string(),
            age
        }
    }
    #[warn(dead_code)]
    fn full_name (&self) -> String {
        format!("{} {}", self.fist_name, self.last_name)
    }
}

struct Account {
    email: String,
    password: String,
    token: String,
    login_from: String
}

impl Account {
    fn new (email: &str, password: &str, token: &str, login_from: &str) -> Account {
        Account {
            email: email.to_string(),
            password: hash_password(password.to_string().as_ref()),
            token: token.to_string(),
            login_from: login_from.to_string()
        }
    }
}

fn hash_password (new_password: &str) -> String { hash(new_password, DEFAULT_COST).unwrap() }
fn verify_password (password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password).unwrap()
}
pub fn run() {
    // Create a new Person
    let person = Person::new("Chuck", "Norris", "male", 38);
    let account = &Account::new("test@account.com", "easypassword", "", "main_app");
    println!("Person {:?}", (person.fist_name, person.last_name, person.gender, person.age));
    println!("Account {:?}", (&account.email, &account.password, &account.token, &account.login_from));
    let hashed = &account.password;
    println!("Testing bcrypt given easypassword is {:?}", verify_password("easypassword", hashed));
    println!("Testing bcrypt given falspassword is {:?}", verify_password("falspassword", hashed));
}

#[test]
pub fn test_create_person() {
    let person = Person::new("Chuck", "Norris", "male", 38);
    assert_eq!(person.fist_name, "Chuck");
    assert_eq!(person.last_name, "Norris");
}

#[test]
pub fn test_create_account() {
    let account = &Account::new("test@account.com", "easypassword", "", "main_app");
    assert_eq!(account.email, "test@account.com");
}
