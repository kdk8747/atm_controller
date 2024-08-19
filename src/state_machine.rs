use rust_fsm::*;

pub enum Input {
    CardInserted,
    PinVerified,
    PinFailed,
    RequestDeposit,
    TargetCashAmountCounted,
    RequestWithdrawal,
    UserInteractionTimeout,
    Successful,
    Unsuccessful,
}

#[derive(Debug)]
pub enum State {
    FatalError,
    Idle,
    WaitingPinNumber,
    Authenticated,
    EjectingCard,
    WaitingCashInput,
    CountingCashForReturn,
    CountingCash,
    EjectingCash,
}

state_machine! {
    #[state_machine(input(super::Input), state(super::State))]
    pub atm(Idle)

    Idle(CardInserted) => WaitingPinNumber,
    WaitingPinNumber => {
        PinVerified => Authenticated,
        PinFailed => EjectingCard,
        UserInteractionTimeout => EjectingCard,
    },
    Authenticated => {
        RequestDeposit => WaitingCashInput,
        RequestWithdrawal => CountingCash,
        UserInteractionTimeout => EjectingCard,
    },
    EjectingCard => {
        Successful => Idle,
        Unsuccessful => FatalError,
    },
    WaitingCashInput => {
        TargetCashAmountCounted => EjectingCard,
        UserInteractionTimeout => CountingCashForReturn,
    },
    CountingCashForReturn => {
        Successful => EjectingCash,
        Unsuccessful => FatalError,
    },
    CountingCash => {
        Successful => EjectingCash,
        Unsuccessful => EjectingCard, // most case: not enough cash
    },
    EjectingCash => {
        Successful => EjectingCard,
        Unsuccessful => FatalError
    },
}