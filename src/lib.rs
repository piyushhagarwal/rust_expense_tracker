// Structure of an expense
pub struct Expense{
    pub id:i32,
    pub name: String,
    pub date: String,
    pub amount: f64
}

pub struct User{
    pub user_name : String,
    pub account_balance : f64,
    pub transactions : Vec<Expense>
}

impl User{

    pub fn new(user_name: String) -> User {
        User{
            user_name,
            account_balance : 0.0,
            transactions : Vec::new()
        }
    }

    pub fn add_balance (&mut self, amount: f64){
        self.account_balance += amount;
    }

    fn add_expense(&mut self, id: i32, name: String, date: String, amount: f64){
        self.transactions.push(
            Expense {
                id,
                name: String::from(name),
                date: String::from(date), 
                amount 
            }
        );
        self.account_balance -= amount;
    }
}


//add expense function 
pub fn add_expense(id: i32, name: String, date: String, amount: f64, expenses_vector: &mut Vec<Expense>){
    expenses_vector.push(Expense {id, name: String::from(name), date: String::from(date), amount: amount });


}

//delete expense function
pub fn delete_expense(id: i32, expenses_vector: &mut Vec<Expense>){
    let mut index = 0;
    for element in expenses_vector.iter(){
        if element.id == id {
            break;
        }
        else{
            index += 1;
        }
    }
    if index < expenses_vector.len() {
        expenses_vector.remove(index);
    }
    else{
        println!("expense not found");
    }
}

//update expense function

pub fn update_expense(id: i32, updated_name: String, updated_date: String, updated_amount: f64, expenses_vector: &mut Vec<Expense>){
    for expense in expenses_vector.iter_mut(){
        if id == expense.id {
            expense.name = updated_name;
            expense.date = updated_date;
            expense.amount = updated_amount;
            return;
        }
    }
    println!("Element not found")
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add_balance(){
        let mut user1 = User::new("piyush".to_string());
        user1.add_balance(100.00);
        assert_eq!(user1.account_balance,100.00);
    }

    #[test]
    fn check_add_expense(){
        let mut user1 = User::new("piyush".to_string());
        user1.add_balance(100.00);
        user1.add_expense(1, "movie".to_string(), "17/07/2023".to_string(), 50.00);
        assert_eq!(user1.account_balance,50.00);
    }
}