use state_machine::{Input, State};

pub mod state_machine;
pub mod bank_api;
pub mod hw_api;


pub struct AtmController {
    state_machine: state_machine::atm::StateMachine,
    pin: String,
    authentication: String,
    requested_cash_input_amount: u64,
    requested_cash_output_amount: u64,
    cash_input_counter: u64,
    error_code: String,
}

impl AtmController {
    pub fn new() -> AtmController {
        AtmController {
            state_machine: state_machine::atm::StateMachine::new(),
            pin: String::new(),
            authentication: String::new(),
            requested_cash_input_amount: 0,
            requested_cash_output_amount: 0,
            cash_input_counter: 0,
            error_code: String::new(),
        }
    }

    pub fn from(state: state_machine::State) -> AtmController {
        AtmController {
            state_machine: state_machine::atm::StateMachine::from_state(state),
            pin: String::new(),
            authentication: String::new(),
            requested_cash_input_amount: 0,
            requested_cash_output_amount: 0,
            cash_input_counter: 0,
            error_code: String::new(),
        }
    }

    pub fn state(&self) -> &State {
        self.state_machine.state()
    }

    pub fn pin(&self) -> &String {
        &self.pin
    }

    pub fn authentication(&self) -> &String {
        &self.authentication
    }

    pub fn request_deposit(&mut self, amount: u64) {
        self.requested_cash_input_amount = amount;
        self.state_machine.consume(&Input::RequestDeposit).unwrap();
    }

    pub fn request_withdrawal(&mut self, amount: u64) {
        self.requested_cash_output_amount = amount;
        self.state_machine.consume(&Input::RequestWithdrawal).unwrap();
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state() {
        let atm = AtmController::new();
        assert_eq!(State::Idle, *atm.state());
    }
}
