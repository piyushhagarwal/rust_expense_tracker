use serde::{Serialize, Deserialize};

pub mod filesystem;

#[derive(Serialize, Deserialize, Debug)]
// Structure of an expense
pub struct Expense{
    pub id:i32,
    pub name: String,
    pub date: String,
    pub amount: f64
}

//Structure of an User
#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub user_name : String,
    pub user_id : i32,
    pub account_balance : f64,
    pub transactions : Vec<Expense>
}

impl User{

    pub fn new(user_name: String, user_id: i32) {
        let file_name = format!("{user_id}.json");
        filesystem::create_file(&file_name).unwrap();
        let user_struct = User{
            user_name,
            user_id,
            account_balance : 0.0,
            transactions : Vec::new()
        };
        let json_string = serde_json::to_string_pretty(&user_struct).unwrap();
        filesystem::write_to_file(&file_name, &json_string).unwrap();
    }

    pub fn add_balance (user_id: i32, amount: f64){
        let file_name = format!("{user_id}.json");
        let json_string = filesystem::read_file(&file_name).unwrap();
        let mut user_struct : User = serde_json::from_str(&json_string).unwrap();
        user_struct.account_balance += amount;
        let json_string = serde_json::to_string_pretty(&user_struct).unwrap();
        filesystem::write_to_file(&file_name, &json_string).unwrap();
    }

    pub fn add_expense(user_id: i32, id: i32, name: String, date: String, amount: f64){
        let file_name = format!("{user_id}.json");
        let json_string = filesystem::read_file(&file_name).unwrap();
        let mut user_struct : User = serde_json::from_str(&json_string).unwrap();
        user_struct.account_balance -= amount;

        user_struct.transactions.push(
            Expense {
                id,
                name: String::from(name),
                date: String::from(date), 
                amount 
            }
        );
        let json_string = serde_json::to_string_pretty(&user_struct).unwrap();
        filesystem::write_to_file(&file_name, &json_string).unwrap();
    }

    pub fn delete_expense(user_id: i32,id: i32){
        let file_name = format!("{user_id}.json");
        let json_string = filesystem::read_file(&file_name).unwrap();
        let mut user_struct : User = serde_json::from_str(&json_string).unwrap();
        let mut index = 0;
        for element in user_struct.transactions.iter(){
            if element.id == id {
                user_struct.account_balance += user_struct.transactions[index].amount;
                user_struct.transactions.remove(index);
                let json_string = serde_json::to_string_pretty(&user_struct).unwrap();
                filesystem::write_to_file(&file_name, &json_string).unwrap();
                return;
            }
            else{
                index += 1;
            }
        }
    }

    pub fn update_expense(user_id: i32, id: i32, updated_name: String, updated_date: String, updated_amount: f64){
        let file_name = format!("{user_id}.json");
        let json_string = filesystem::read_file(&file_name).unwrap();
        let mut user_struct : User = serde_json::from_str(&json_string).unwrap();
        for expense in user_struct.transactions.iter_mut(){
            if id == expense.id {
                user_struct.account_balance += expense.amount;
                expense.name = updated_name;
                expense.date = updated_date;
                expense.amount = updated_amount;
                user_struct.account_balance -= expense.amount;
                let json_string = serde_json::to_string_pretty(&user_struct).unwrap();
                filesystem::write_to_file(&file_name, &json_string).unwrap();
                return;
            }
        }
    }
}
