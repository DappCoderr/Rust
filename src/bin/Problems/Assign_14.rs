struct User{
    name: String,
    age: u32
}

trait Summary {
    fn summaries(&self) -> String{
        return String::from("hi there!");
    }
}

impl Summary for User {
    fn summaries(&self) -> String {
        return format!("Your name is {} and age is {}", self.name, self.age);
    }
}

fn main(){
    let user = User{
        name: String::from("dc"),
        age: 32,
    };
    println!("{}",user.summaries());
}