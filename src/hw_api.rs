use crate::AtmController;


impl AtmController {

    fn eject_card(&mut self) -> bool {
        // via hw API
        self.error_code = String::from("Some Error Code from HW");
        // OR
        true
    }

    fn count_cash(&mut self) -> bool {
        // via hw API
        self.error_code = String::from("Some Error Code from HW");
        // OR
        true
    }

    fn eject_cash(&mut self) -> bool {
        // via hw API
        self.error_code = String::from("Some Error Code from HW");
        // OR
        true
    }
}