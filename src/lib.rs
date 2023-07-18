mod filesystem;

// Structure of an expense
pub struct Expense{
    pub id:i32,
    pub name: String,
    pub date: String,
    pub amount: f64
}

//Structure of an User
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

    pub fn add_expense(&mut self, id: i32, name: String, date: String, amount: f64){
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

    pub fn delete_expense(&mut self,id: i32){
        let mut index = 0;
        for element in self.transactions.iter(){
            if element.id == id {
                self.account_balance += self.transactions[index].amount;
                self.transactions.remove(index);
                break;
            }
            else{
                index += 1;
            }
        }
    }

    pub fn update_expense(&mut self, id: i32, updated_name: String, updated_date: String, updated_amount: f64){
        for expense in self.transactions.iter_mut(){
            if id == expense.id {
                self.account_balance += expense.amount;
                expense.name = updated_name;
                expense.date = updated_date;
                expense.amount = updated_amount;
                self.account_balance -= expense.amount;
                return;
            }
        }
    }
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
        assert_eq!(user1.transactions[0].id, 1);
        assert_eq!(user1.account_balance,50.00);
    }

    #[test]
    fn check_delete_expense(){
        let mut user1 = User::new("piyush".to_string());
        user1.add_balance(100.00);
        user1.add_expense(1, "movie".to_string(), "17/07/2023".to_string(), 50.00);
        user1.add_expense(2, "travel".to_string(), "18/07/2023".to_string(), 20.00);
        user1.delete_expense(1);
        assert_eq!(user1.account_balance,80.00);
        assert_eq!(user1.transactions[0].id, 2);
    }

    #[test]
    fn check_update_expense(){
        let mut user1 = User::new("piyush".to_string());
        user1.add_balance(100.00);
        user1.add_expense(1, "movie".to_string(), "17/07/2023".to_string(), 50.00);
        user1.update_expense(1,"traval".to_string(), "18/07/2023".to_string(), 60.00);
        assert_eq!(user1.account_balance,40.00);
        assert_eq!(user1.transactions[0].amount, 60.00);
    }
}