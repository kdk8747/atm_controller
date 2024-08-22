use crate::{state_machine::Input, AtmController};

pub enum HwEvent {
    CardInserted,
    CardEjected,
    CardEjectionTimeout,
    CashInserted,
    CashCountSuccess,
    CashCountFailed,
    CashEjected,
    CashEjectionTimeout,
    UserInteractionTimeout,
}

pub enum HwEventArg {
    Count(u64),
    Pin(String),
    None(),
}

impl HwEventArg {
    fn pin(self) -> Option<String> {
        match self {
            HwEventArg::Pin(c) => Some(c),
            _ => None,
        }
    }
}

impl AtmController {

    pub fn eject_card(&mut self) -> bool {
        // via hw API
        self.error_code = String::from("Some Error Code from HW");
        // OR
        true
    }

    pub fn count_output_cash(&mut self) -> bool {
        // using arg
        self.requested_cash_output_amount;
        // via hw API
        self.error_code = String::from("Some Error Code from HW");
        // OR
        true
    }

    pub fn eject_cash(&mut self) -> bool {
        // via hw API
        self.error_code = String::from("Some Error Code from HW");
        // OR
        true
    }

    pub fn receive_event(&mut self, event: HwEvent, arg: HwEventArg) {
        match event {
            HwEvent::CardInserted => {
                self.state_machine.consume(&Input::CardInserted).unwrap();
                self.pin = arg.pin().unwrap();
                self.get_authentication();
            }
            HwEvent::CardEjected | HwEvent::CashCountSuccess | HwEvent::CashEjected => {
                self.state_machine.consume(&Input::Successful).unwrap();
            }
            HwEvent::CardEjectionTimeout | HwEvent::CashCountFailed | HwEvent::CashEjectionTimeout => {
                self.state_machine.consume(&Input::Unsuccessful).unwrap();
            }
            HwEvent::CashInserted => {
                self.cash_input_counter += 1;
                if self.cash_input_counter == self.requested_cash_input_amount {
                    self.requested_cash_input_amount = 0;
                    self.cash_input_counter = 0;
                    self.state_machine.consume(&Input::TargetCashAmountCounted).unwrap();
                }
            }
            HwEvent::UserInteractionTimeout => {
                self.state_machine.consume(&Input::UserInteractionTimeout).unwrap();
            }
        }
    }
}