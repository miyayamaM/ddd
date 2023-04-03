pub mod user;
pub mod value_objects;

use user::User;
use value_objects::user_id::UserId;

fn main() {
    let mut user = User::new(UserId(1), "xxxxx".to_string());
    user.change_user_name("aasss".to_string());
    println!("{:?}", user);
}
