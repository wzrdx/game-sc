const COMMON_ENERGY_PER_S: u64 = 278 * 3;
const UNCOMMON_ENERGY_PER_S: u64 = 278 * 4;
const RARE_ENERGY_PER_S: u64 = 278 * 6;
const ROYALS_ENERGY_PER_S: u64 = 278 * 8;
const ONEOFONE_ENERGY_PER_S: u64 = 278 * 10;
const ELDER_ENERGY_PER_S: u64 = 278 * 9;

multiversx_sc::imports!();

use crate::storage;

#[multiversx_sc::module]
pub trait Helpers: storage::Storage {
    fn get_token_mapper(&self, index: usize) -> FungibleTokenMapper {
        let mapper;

        if index == 0 {
            mapper = self.energy_mapper();
        } else if index == 1 {
            mapper = self.herbs_mapper();
        } else if index == 2 {
            mapper = self.gems_mapper();
        } else {
            mapper = self.essence_mapper();
        }

        mapper
    }

    fn claim_staking_rewards_for_user(&self, user: &ManagedAddress) {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let reward = self.get_staking_rewards(user);
        self.last_staking_timestamp(user).set(current_timestamp);

        if reward > 0 {
            self.energy_mapper().mint_and_send(user, reward);
        }
    }

    fn get_staking_rewards(&self, user: &ManagedAddress) -> BigUint {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let last_timestamp = self.last_staking_timestamp(user).get();

        if last_timestamp == 0 || current_timestamp <= last_timestamp {
            return BigUint::zero();
        }

        let block_diff: u64 = current_timestamp - last_timestamp;
        let travelers_rewards = BigUint::from(block_diff * self.get_travelers_yield(user));

        let elder_count: u64 = self.staked_elder_nonces(user).len() as u64;
        let elders_rewards = BigUint::from(block_diff * ELDER_ENERGY_PER_S * elder_count);

        travelers_rewards + elders_rewards
    }

    fn get_travelers_yield(&self, user: &ManagedAddress) -> u64 {
        let mut travelers_rewards: u64 = 0;

        for nonce in self.staked_traveler_nonces(user).iter() {
            travelers_rewards += self.get_energy_yield(self.rarity_class(nonce).get());
        }

        travelers_rewards
    }

    fn get_energy_yield(&self, rarity_class: u8) -> u64 {
        let mut energy_yield: u64 = 0;

        if rarity_class == 1 {
            energy_yield = COMMON_ENERGY_PER_S;
        } else if rarity_class == 2 {
            energy_yield = UNCOMMON_ENERGY_PER_S;
        } else if rarity_class == 3 {
            energy_yield = RARE_ENERGY_PER_S;
        } else if rarity_class == 4 {
            energy_yield = ROYALS_ENERGY_PER_S;
        } else if rarity_class == 5 {
            energy_yield = ONEOFONE_ENERGY_PER_S;
        }

        energy_yield
    }

    fn build_uris_vec(&self) -> ManagedVec<ManagedBuffer> {
        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new_from_bytes(
            "https://ipfs.io/ipfs/bafybeiezcv3mihkkzoug3swhfghwoggc5i7kuocbekeyndiensdlqagoyy".as_bytes(),
        ));

        uris
    }

    fn build_attributes_buffer(&self) -> ManagedBuffer {
        let mut attributes = ManagedBuffer::new();
        attributes.append(&ManagedBuffer::new_from_bytes(
            "tags:Home X;metadata:bafkreigrkib7uq72s2j232x3pbj34gs6sbtg2vaotv2433tsljfjevx3zu".as_bytes(),
        ));

        attributes
    }

    fn to_egld(&self, value: u64) -> u64 {
        value.mul(1000000000000000000 as u64)
    }

    fn require_conditions(&self) {
        require!(!self.is_game_paused().get(), "The game is temporarily paused");
    }
}
