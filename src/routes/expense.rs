use axum::{Json, extract::Path};
use serde::{Serialize, Deserialize};

use crate::{filesystem, user::{User, Expense}};

//Api to get a particular user by its id
pub async fn get_user(Path(user_id): Path<i32>) -> Json<User>{
    let file_name = format!("{}.json",user_id);
    let json_string = filesystem::read_file(&file_name).unwrap();
    let user_struct : User = serde_json::from_str(&json_string).unwrap();
    Json(user_struct)
}

//Api to create User
#[derive(Deserialize)]
pub struct BodyCreateUser{
    user_name: String,
    user_id: i32
}

pub async fn create_user(body: Json<BodyCreateUser>) -> Json<User> {
    User::new(body.user_name.to_string(), body.user_id);
    let file_name = format!("{}.json",body.user_id);
    let json_string = filesystem::read_file(&file_name).unwrap();
    let user_struct : User = serde_json::from_str(&json_string).unwrap();
    Json(user_struct)
}

//Api to add add balance
#[derive(Serialize,Deserialize)]
pub struct BodyAddBalance{
    amount: f64
}
pub async fn add_balance(Path(user_id): Path<i32>,body: Json<BodyAddBalance>) -> Json<User>{
    User::add_balance(user_id, body.amount);
    let file_name = format!("{}.json",user_id);
    let json_string = filesystem::read_file(&file_name).unwrap();
    let user_struct : User = serde_json::from_str(&json_string).unwrap();
    Json(user_struct)
}

//Api for adding an expense 
pub async fn add_expense(Path(user_id): Path<i32>,body: Json<Expense>) -> Json<User>{
    User::add_expense(user_id, body.id, body.name.to_string(), body.date.to_string(), body.amount);
    let file_name = format!("{}.json",user_id);
    let json_string = filesystem::read_file(&file_name).unwrap();
    let user_struct : User = serde_json::from_str(&json_string).unwrap();
    Json(user_struct)
}

//Api for deleting an expense

#[derive(Deserialize)]
pub struct BodyDeleteExpense{
    id: i32
}

pub async fn delete_expense(Path(user_id): Path<i32>,body: Json<BodyDeleteExpense>) -> Json<User>{
    User::delete_expense(user_id, body.id);
    let file_name = format!("{}.json",user_id);
    let json_string = filesystem::read_file(&file_name).unwrap();
    let user_struct : User = serde_json::from_str(&json_string).unwrap();
    Json(user_struct)
}

//Api for updating an expense 
pub async fn update_expense(Path(user_id): Path<i32>,body: Json<Expense>) -> Json<User>{
    User::update_expense(user_id, body.id, body.name.to_string(), body.date.to_string(), body.amount);
    let file_name = format!("{}.json",user_id);
    let json_string = filesystem::read_file(&file_name).unwrap();
    let user_struct : User = serde_json::from_str(&json_string).unwrap();
    Json(user_struct)
}

