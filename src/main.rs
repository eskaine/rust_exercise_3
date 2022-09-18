struct UserAccount {
    name: String,
    age: Option<u32>,
}

trait Balance {
    fn get_balance(&self) -> u32 {
        10
    }
}

// implement trait Balance to UserAccount struct
impl Balance for UserAccount {}

fn increase_balance<T: Balance>(account: &T, amount: u32) -> Result<u32, String> {
    if amount <= 10 {
        Ok(account.get_balance() + amount)
    } else {
        Err("Increase must be less than 10!".to_owned())
    }
}

fn main() {
    let user = UserAccount {
        name: "John".to_owned(),
        age: Some(10),
    };

    let balance = increase_balance(&user, 11);

    match balance {
        Ok(amount) => println!("UserAccount balance increased to {}", amount),
        Err(e) => println!("Error: {}", e),
    }

    if let Some(age) = user.age {
        println!("User is {}", user.name);
        println!("User age is {}", age);
    }
}
