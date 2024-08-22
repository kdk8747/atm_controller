use crate::AtmController;


pub struct BalanceRow {
    pub datetime_iso8601: String,
    pub deposit: u64,
    pub withdrawal: u64,
    pub balance: u64,
}

impl AtmController {

    pub fn get_authentication(&mut self) -> bool {
        // TODO: async
        // using arg
        //self.pin.clone();
        // via bank system API
        self.error_code = String::from("Request Timeout");
        // OR
        self.error_code = String::from("Wrong Pin Number");
        // OR
        self.error_code = String::from("Some Error Code from Bank API");
        //self.state_machine.consume(&Input::PinFailed).unwrap();
        // OR
        self.authentication = String::from("Some Auth from Bank API"); // auth string
        //self.state_machine.consume(&Input::PinVerified).unwrap();
        todo!();
    }

    pub fn get_account_no(&mut self) -> Vec<String> {
        // TODO: async
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

    pub fn get_account_balance(&mut self, _account_no: &String) -> Vec<BalanceRow> {
        // TODO: async
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