use rust_expense_tracker::{User,filesystem};

#[test]
    fn test_new_user(){
        let user_id = 1;
        User::new("piyush".to_string(), user_id);
        assert!(filesystem::open_file("1.json").is_ok());
        let content = "{\n  \"user_name\": \"piyush\",\n  \"user_id\": 1,\n  \"account_balance\": 0.0,\n  \"transactions\": []\n}";
        assert_eq!(filesystem::read_file("1.json").unwrap(),content);
        
        //Delete the test file after testing
        filesystem::delete_file("1.json").unwrap();
    }

    #[test]
    fn test_add_balance(){
        let user_id = 2;
        User::new("piyush".to_string(), user_id);
        User::add_balance(user_id,300.00);
        let content = "{\n  \"user_name\": \"piyush\",\n  \"user_id\": 2,\n  \"account_balance\": 300.0,\n  \"transactions\": []\n}";
        assert_eq!(filesystem::read_file("2.json").unwrap(),content);

        //Delete the test file after testing
        filesystem::delete_file("2.json").unwrap();
    }

    #[test]
    fn test_add_expense(){
        let user_id = 3;
        User::new("piyush".to_string(),user_id);
        User::add_balance(user_id,300.00);
        User::add_expense(user_id,2, "travel".to_string(), "19/07/2023".to_string(), 100.00);
        let content = "{\n  \"user_name\": \"piyush\",\n  \"user_id\": 3,\n  \"account_balance\": 200.0,\n  \"transactions\": [\n    {\n      \"id\": 2,\n      \"name\": \"travel\",\n      \"date\": \"19/07/2023\",\n      \"amount\": 100.0\n    }\n  ]\n}";
        assert_eq!(filesystem::read_file("3.json").unwrap(),content);

        //Delete the test file after testing
        filesystem::delete_file("3.json").unwrap();
    }

    #[test]
    fn test_delete_expense(){
        let user_id = 4;
        User::new("piyush".to_string(),user_id);
        User::add_balance(user_id,300.00);
        User::add_expense(user_id,1, "movie".to_string(), "17/07/2023".to_string(), 50.00);
        User::add_expense(user_id,2, "travel".to_string(), "19/07/2023".to_string(), 100.00);
        User::delete_expense(user_id,2);
        let content = "{\n  \"user_name\": \"piyush\",\n  \"user_id\": 4,\n  \"account_balance\": 250.0,\n  \"transactions\": [\n    {\n      \"id\": 1,\n      \"name\": \"movie\",\n      \"date\": \"17/07/2023\",\n      \"amount\": 50.0\n    }\n  ]\n}";
        assert_eq!(filesystem::read_file("4.json").unwrap(),content);

        //Delete the test file after testing
        filesystem::delete_file("4.json").unwrap();
    }

    #[test]
    fn test_update_expense(){
        let user_id = 5;
        User::new("piyush".to_string(),user_id);
        User::add_balance(user_id,300.00);
        User::add_expense(user_id,1, "movie".to_string(), "17/07/2023".to_string(), 50.00);
        User::add_expense(user_id,2, "travel".to_string(), "19/07/2023".to_string(), 100.00);
        User::update_expense(user_id,2,"traval".to_string(), "18/07/2023".to_string(), 60.00);
        let content = "{\n  \"user_name\": \"piyush\",\n  \"user_id\": 5,\n  \"account_balance\": 190.0,\n  \"transactions\": [\n    {\n      \"id\": 1,\n      \"name\": \"movie\",\n      \"date\": \"17/07/2023\",\n      \"amount\": 50.0\n    },\n    {\n      \"id\": 2,\n      \"name\": \"traval\",\n      \"date\": \"18/07/2023\",\n      \"amount\": 60.0\n    }\n  ]\n}";
        assert_eq!(filesystem::read_file("5.json").unwrap(),content);

        //Delete the test file after testing
        filesystem::delete_file("5.json").unwrap();
    }