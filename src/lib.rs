#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Symbol, Vec, contracttype};

#[derive(Clone, Copy)]
#[contracttype]
pub enum Status {
    Open,
    Closed,
}

#[contracttype]
#[derive(Clone)]
pub struct DonationEvent {
    pub owner: Address,
    pub token_reward: Address,
    pub token_donation: Address,
    pub amount_raised: i128,
    pub goal_amount: i128,
    pub deadline: u64,
    pub status: Status,
}

const EVENTS: Symbol = symbol_short!("EVENTS");

#[contract]
pub struct DonatifiContract;

#[contractimpl]
impl DonatifiContract {
    pub fn create_evt(
        env: Env,
        owner: Address,
        token_reward: Address,
        token_donation: Address,
        goal_amount: i128,
        deadline: u64,
    ) -> u32 {
        owner.require_auth();

        let mut events: Vec<DonationEvent> = env.storage().instance().get(&EVENTS).unwrap_or(vec![&env]);

        let event = DonationEvent {
            owner,
            token_reward,
            token_donation,
            amount_raised: 0,
            goal_amount,
            deadline,
            status: Status::Open,
        };

        events.push_back(event);
        env.storage().instance().set(&EVENTS, &events);
        events.len() - 1
    }

    pub fn get_event(env: Env, id: u32) -> Option<DonationEvent> {
        let events: Vec<DonationEvent> = env.storage().instance().get(&EVENTS).unwrap_or(vec![&env]);
        events.get(id)
    }

    pub fn list_events(env: Env) -> Vec<DonationEvent> {
        env.storage().instance().get(&EVENTS).unwrap_or(vec![&env])
    }

    pub fn donate(env: Env, donor: Address, id: u32, amount: i128) {
        donor.require_auth();

        let mut events: Vec<DonationEvent> = env.storage().instance().get(&EVENTS).unwrap_or(vec![&env]);
        let mut event = events.get(id).expect("Event not found");

        assert!(matches!(event.status, Status::Open), "Event is closed");
        assert!(env.ledger().timestamp() <= event.deadline, "Event has expired");

        event.amount_raised += amount;

        if event.amount_raised >= event.goal_amount {
            event.status = Status::Closed;
        }

        events.set(id, event);
        env.storage().instance().set(&EVENTS, &events);
    }
}
