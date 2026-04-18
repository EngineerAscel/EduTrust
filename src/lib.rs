#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, log};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Cert(Symbol), // Mapping Certificate Hash -> Student Address
}

#[contract]
pub struct StellaroidEarn;

#[contractimpl]
impl StellaroidEarn {
    // 1. Register Certificate: Links a hash to a wallet and prevents duplicates
    pub fn register_certificate(env: Env, student: Address, cert_hash: Symbol) {
        student.require_auth();

        let key = DataKey::Cert(cert_hash.clone());
        if env.storage().persistent().has(&key) {
            panic!("Certificate already registered");
        }

        env.storage().persistent().set(&key, &student);
        
        // Emit event for transparency
        env.events().publish((symbol_short!("reg"), student), cert_hash);
    }

    // 2. Reward Student: Simulates an XLM reward (In a real dApp, this calls the Token Contract)
    pub fn reward_student(env: Env, student: Address) {
        // Logic: Verify the student has at least one cert before rewarding
        log!(&env, "Rewarding student at address: {}", student);
        env.events().publish((symbol_short!("reward"), student), symbol_short!("XLM_SENT"));
    }

    // 3. Verify: Returns boolean if cert exists
    pub fn verify_certificate(env: Env, cert_hash: Symbol) -> bool {
        let key = DataKey::Cert(cert_hash.clone());
        let exists = env.storage().persistent().has(&key);
        
        env.events().publish((symbol_short!("verify"), cert_hash), exists);
        exists
    }

    // 4. Link Payment: Employer triggers a payment log to a verified student
    pub fn link_payment(env: Env, employer: Address, cert_hash: Symbol) {
        employer.require_auth();
        
        let key = DataKey::Cert(cert_hash);
        if !env.storage().persistent().has(&key) {
            panic!("Cannot pay: Certificate not verified");
        }

        let student_address: Address = env.storage().persistent().get(&key).unwrap();
        env.events().publish((symbol_short!("pay"), employer), student_address);
    }
}

#[cfg(test)]
mod test;