use cosmwasm_std::Env;
use secret_toolkit::crypto::ContractPrng;

pub struct Random {
    prng: ContractPrng,
    current_byte: usize,
    random_bytes: [u8; 32],
}

impl Random {
    pub fn new(env: &Env) -> Self {
        let mut prng = ContractPrng::from_env(env);
        let random_bytes = prng.rand_bytes();
        Self { prng, current_byte: 0, random_bytes }
    }

    pub fn get_random_number(&mut self, min: u8, max: u8) -> u8 {
        if self.current_byte >= 32 {
            self.random_bytes = self.prng.rand_bytes();
            self.current_byte = 0;
        }
        let byte = self.random_bytes[self.current_byte];
        self.current_byte += 1;
        min + (byte % (max - min + 1))
    }

    pub fn get_random_number_up_to(&mut self, max: u8) -> u8 {
        self.get_random_number(0, max)
    }
}
