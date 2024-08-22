use atm_controller::*;
use atm_controller::hw_api::*;
use state_machine::State;

#[test]
#[should_panic] // get_authentication() not implemented
fn card_insertion() {
    let mut atm = AtmController::new();
    atm.receive_event(HwEvent::CardInserted, HwEventArg::Pin(String::from("0345235")));
    assert_eq!(State::WaitingPinNumber, *atm.state());
    assert_eq!(String::from("0345235"), *atm.pin());
    assert_eq!(String::from("Some Auth from Bank API"), *atm.authentication());
    // TODO: API mock-up test
}

#[test]
fn request_deposit() {
    let mut atm = AtmController::from(State::Authenticated);
    atm.request_deposit(3);
    assert_eq!(State::WaitingCashInput, *atm.state());
    atm.receive_event(HwEvent::CashInserted, HwEventArg::None());
    assert_eq!(State::WaitingCashInput, *atm.state());
    atm.receive_event(HwEvent::CashInserted, HwEventArg::None());
    assert_eq!(State::WaitingCashInput, *atm.state());
    atm.receive_event(HwEvent::CashInserted, HwEventArg::None());
    assert_eq!(State::EjectingCard, *atm.state());
}

#[test]
fn request_withdrawal() {
    let mut atm = AtmController::from(State::Authenticated);
    atm.request_withdrawal(3);
    assert_eq!(State::CountingCash, *atm.state());
}