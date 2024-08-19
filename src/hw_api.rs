use crate::{state_machine::Input, AtmController};

enum HwEvent {
    CardInserted,
    CardEjected,
    CardEjectionTimeout,
    CashInserted,
    CashCountSuccess,
    CashCountFailed,
    CashEjected,
    CashEjectionTimeout,
}

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

    fn event_received(&mut self, event: HwEvent) {
        match event {
            HwEvent::CardInserted => {
                self.reset_user_interaction_timestamp();
                self.state_machine.consume(&Input::CardInserted).unwrap();
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
            _ => unreachable!()
        }
    }
}