//! This contract implements simple counter backed by storage on blockchain.
//!
//! The contract provides methods to [increment] / [decrement] counter and
//! get it's current value [get_num] or [reset].
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: u16,
    top_threshold: i8,
    low_threshold: i8,
}

#[near_bindgen]
impl Counter {
    /// Public method: Returns the counter value.
    pub fn get_num(&self) -> u16 {
        return self.val;
    }

    /// Public method: Increment the counter.
    pub fn increment(&mut self) {
        if self.val < self.top_threshold {
            self.val += 1;
        }
        log!("Increased number to {}", self.val);
    }

    /// Public method: Decrement the counter.
    pub fn decrement(&mut self) {
        if self.val > self.lower_threshold {
            self.val -= 1;
        }   
        log!("Decreased number to {}", self.val);
    }

    /// Public method - Reset to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        log!("Reset counter to zero");
    }

    pub fn update_top_threshold(&mut self,  value: i8){
        self.top_threshold = value
        log!("Update top threshold to {}", self.top_threshold);
    }
    pub fn update_low_threshold(&mut self,  value: i8){
        self.low_threshold = value
        log!("Update low threshold to {}", self.low_threshold);
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be: `cargo test`
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment() {
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter { val: 0 , top_threshold: 0, low_threshold: 0};
        contract.increment();
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let mut contract = Counter { val: 0 , top_threshold: 0, low_threshold: 0};
        contract.decrement();
        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn increment_and_reset() {
        let mut contract = Counter { val: 0 , top_threshold: 0, low_threshold: 0};
        contract.increment();
        contract.reset();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    fn update_top_threshold() {
        let mut contract = Counter { val: 0 , top_threshold: 0, low_threshold: 0};
        contract.update_top_threshold(100)
        contract.reset();
    }
    
    #[test]
    fn update_low_threshold() {
        let mut contract = Counter { val: 0 , top_threshold: 0, low_threshold: 0};
        contract.update_low_threshold(-100)
        contract.reset();
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let mut contract = Counter { val: 127 };
        contract.increment();
    }

    #[test]
    #[should_panic]
    fn panics_on_underflow() {
        let mut contract = Counter { val: -128 };
        contract.decrement();
    }
}
