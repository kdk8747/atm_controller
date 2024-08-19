pub mod state_machine;
pub mod bank_api;
pub mod hw_api;


struct AtmController {
    state_machine: state_machine::atm::StateMachine,
    authentication: String,
    requested_cash_input_amount: u64,
    requested_cash_output_amount: u64,
    error_code: String,
}

impl AtmController {
    pub fn new() -> AtmController {
        AtmController {
            state_machine: state_machine::atm::StateMachine::new(),
            authentication: String::new(),
            requested_cash_input_amount: 0,
            requested_cash_output_amount: 0,
            error_code: String::new(),
        }
    }
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
