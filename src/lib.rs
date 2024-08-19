use std::{ops::Add, time::{Duration, SystemTime, UNIX_EPOCH}};

use state_machine::{Input, State};

pub mod state_machine;
pub mod bank_api;
pub mod hw_api;


struct AtmController {
    state_machine: state_machine::atm::StateMachine,
    authentication: String,
    requested_cash_input_amount: u64,
    requested_cash_output_amount: u64,
    cash_input_counter: u64,
    user_interaction_timestamp: Duration,
    error_code: String,
}

impl AtmController {
    pub fn new() -> AtmController {
        AtmController {
            state_machine: state_machine::atm::StateMachine::new(),
            authentication: String::new(),
            requested_cash_input_amount: 0,
            requested_cash_output_amount: 0,
            cash_input_counter: 0,
            user_interaction_timestamp: Duration::new(0,0),
            error_code: String::new(),
        }
    }

    pub fn reset_user_interaction_timestamp(&mut self) {
        self.user_interaction_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    }

    pub fn request_deposit(&mut self, amount: u64) {
        self.requested_cash_output_amount = amount;
        self.state_machine.consume(&Input::RequestDeposit).unwrap();
    }

    pub fn request_withdrawal(&mut self, amount: u64) {
        self.requested_cash_input_amount = amount;
        self.state_machine.consume(&Input::RequestWithdrawal).unwrap();
    }

    pub fn machine_loop_run(&mut self) {
        match self.state_machine.state() {
            State::WaitingPinNumber | State::Authenticated | State::WaitingCashInput => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                if self.user_interaction_timestamp+Duration::new(30,0) < now {
                    self.state_machine.consume(&Input::UserInteractionTimeout).unwrap();
                }
            }
            _ => {},
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
