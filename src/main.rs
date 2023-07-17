use rust_expense_tracker::Expense;
fn main() {
    let mut expenses: Vec<Expense> = Vec::new();

    rust_expense_tracker::add_expense(1, String::from("test"), String::from("test"), 60.00, &mut expenses);
    rust_expense_tracker::add_expense(2, String::from("test"), String::from("test"), 60.00, &mut expenses);
    rust_expense_tracker::add_expense(3, String::from("test"), String::from("test"), 60.00, &mut expenses);
    rust_expense_tracker::add_expense(4, String::from("test"), String::from("test"), 60.00, &mut expenses);

    rust_expense_tracker::delete_expense(4, &mut expenses);

    rust_expense_tracker::update_expense(1, String::from("test1"), String::from("test1"), 80.00, &mut expenses);



    for expense in &expenses{
        println!("{} {} {} {}",expense.id,expense.name,expense.date,expense.amount)
    }
    println!()
}
