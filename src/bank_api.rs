use crate::AtmController;


pub struct BalanceRow {
    datetime_iso8601: String,
    deposit: u64,
    withdrawal: u64,
    balance: u64,
}

impl AtmController {

    fn get_authentication(&mut self, _pin: &String) -> bool {
        // via bank system API
        self.error_code = String::from("Request Timeout");
        // OR
        self.error_code = String::from("Wrong Pin Number");
        // OR
        self.error_code = String::from("Some Error Code from Bank API");
        // OR
        self.authentication = String::from("Some Auth from Bank API"); // auth string
        todo!();
    }

    pub async fn get_account_no(&mut self) -> Vec<String> {
        let _auth = &self.authentication;
        // via bank system API
        self.error_code = String::from("Request Timeout");
        // OR
        self.error_code = String::from("Authentication Expired");
        // OR
        self.error_code = String::from("Some Error Code from Bank API");
        // OR
        todo!();
    }

    pub async fn get_account_balance(&mut self, _account_no: &String) -> Vec<BalanceRow> {
        let _auth = &self.authentication;
        // via bank system API
        self.error_code = String::from("Request Timeout");
        // OR
        self.error_code = String::from("Authentication Expired");
        // OR
        self.error_code = String::from("Some Error Code from Bank API");
        // OR
        todo!();
    }
}