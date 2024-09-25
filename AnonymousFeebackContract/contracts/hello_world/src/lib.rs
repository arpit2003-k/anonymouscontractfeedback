#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Vec, Symbol, String};

// Define the contract struct
#[contract]
pub struct FeedbackContract;

// Implementation of the contract
#[contractimpl]
impl FeedbackContract {
    
    // Function to send anonymous feedback
    pub fn send_feedback(env: Env, feedback: String) {
        // Fetch the existing feedback list from storage or initialize a new one
        let key = Symbol::new(&env, "feedbacks");
        let mut feedbacks: Vec<String> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        // Add the new feedback to the list
        feedbacks.push_back(feedback);

        // Store the updated list back in the storage
        env.storage().instance().set(&key, &feedbacks);
    }

    // Function to retrieve all anonymous feedbacks
    pub fn retrieve_feedbacks(env: Env) -> Vec<String> {
        // Fetch the feedbacks from storage
        let key = Symbol::new(&env, "feedbacks");
        let feedbacks: Vec<String> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        // Return the feedbacks
        feedbacks
    }
}
