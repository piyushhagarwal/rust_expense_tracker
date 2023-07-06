// Structure of an expense
struct Expense{
    id:i32,
    name: String,
    date: String,
    amount: f64
}

//add expense function 
fn add_expense(id: i32, name: String, date: String, amount: f64, expenses_vector: &mut Vec<Expense>){
    expenses_vector.push(Expense {id:id, name: String::from(name), date: String::from(date), amount: amount });

}

//delete expense function
fn delete_expense(id: i32, expenses_vector: &mut Vec<Expense>){
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

fn update_expense(id: i32, updated_name: String, updated_date: String, updated_amount: f64, expenses_vector: &mut Vec<Expense>){
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


fn main() {
    let mut expenses: Vec<Expense> = Vec::new();

    add_expense(1, String::from("test"), String::from("test"), 60.00, &mut expenses);
    add_expense(2, String::from("test"), String::from("test"), 60.00, &mut expenses);
    add_expense(3, String::from("test"), String::from("test"), 60.00, &mut expenses);
    add_expense(4, String::from("test"), String::from("test"), 60.00, &mut expenses);

    delete_expense(5, &mut expenses);

    update_expense(1, String::from("test1"), String::from("test1"), 80.00, &mut expenses);



    for expense in &expenses{
        println!("{} {} {} {}",expense.id,expense.name,expense.date,expense.amount)
    }
    println!()
}
