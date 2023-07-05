// Structure of an expense
struct Expense{
    name: String,
    date: String,
    amount: f64
}

// Create a vector of expenses

//add expense function 
fn add_expense(name: String, date: String, amount: f64, expenses_vector: &mut Vec<Expense>){
    expenses_vector.push(Expense { name: String::from(name), date: String::from(date), amount: amount });

}

//delete expense function

//update expense function


fn main() {
    let mut expenses: Vec<Expense> = Vec::new();

    add_expense(String::from("test"), String::from("test"), 60.00, &mut expenses);

    for expense in &expenses{
        println!("{} {} {}",expense.name,expense.date,expense.amount)
    }
    println!()
}
