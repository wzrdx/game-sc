multiversx_sc::imports!();

use crate::interface::*;
use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Quests: storage::Storage + helpers::Helpers {
    #[only_owner]
    #[endpoint(setQuests)]
    fn set_quests(&self, quests: ManagedVec<Quest<Self::Api>>) {
        self.quests().clear();

        for quest in quests.iter() {
            self.quests().push(&quest);
        }
    }

    #[only_owner]
    #[endpoint(clearOngoingQuests)]
    fn clear_ongoing_quests(&self) {
        for address in self.active_players().iter() {
            self.ongoing_quests(&address).clear();
        }

        self.active_players().clear();
    }

    #[only_user_account]
    #[payable("*")]
    #[endpoint(startQuest)]
    fn start_quest(&self, id: u8) {
        self.require_conditions();
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        for ongoing_quest in self.ongoing_quests(&caller).iter() {
            require!(id != ongoing_quest.id, "Cannot start an already ongoing quest");
        }

        let quest = self.quests().get(id as usize);
        let quest_duration = quest.duration as u64;
        let trial_timestamp = self.trial_timestamp().get();

        require!(
            quest_duration + current_timestamp < trial_timestamp,
            "Quest duration exceeds end of Trial"
        );

        // Payment check & burning of tokens
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers();
        let requirements = &quest.requirements;

        require!(
            payments.len() == requirements.iter().filter(|&x| x > 0).count(),
            "Received incorrect number of payments"
        );

        let mut payment_index: usize = 0;

        for (i, requirement) in requirements.iter().enumerate() {
            if requirement > 0 {
                let payment = payments.get(payment_index);
                let mapper = self.get_token_mapper(i);

                mapper.require_same_token(&payment.token_identifier);
                require!(payment.amount == BigUint::from(requirement), "Incorrect payment");
                mapper.burn(&payment.amount);

                payment_index += 1;
            }
        }

        // Add to ongoing quests
        self.ongoing_quests(&caller).push(&OngoingQuest {
            id,
            end_timestamp: current_timestamp + (quest.duration as u64),
        });

        self.active_players().insert(caller.clone());
    }

    #[only_user_account]
    #[endpoint(completeQuest)]
    fn complete_quest(&self, id: u8) {
        let caller = self.blockchain().get_caller();

        let mut search_result: Option<OngoingQuest> = None;
        let mut index_to_remove: usize = 0;

        for (i, q) in self.ongoing_quests(&caller).iter().enumerate() {
            if q.id == id {
                search_result = Some(q);
                index_to_remove = i + 1;
            }
        }

        let ongoing_quest = match search_result {
            Some(q) => q,
            None => sc_panic!("Ongoing quest not found"),
        };

        let current_timestamp = self.blockchain().get_block_timestamp();

        require!(current_timestamp >= ongoing_quest.end_timestamp, "Quest cannot be completed yet");

        // Rewards
        let quest = self.quests().get(id as usize);
        let rewards = &quest.rewards;

        // Ticket
        if quest.is_final {
            let tickets_amount: u64 = rewards.iter().sum();
            self.tickets_mapper()
                .nft_add_quantity_and_send(&caller, 1 as u64, BigUint::from(tickets_amount));

            let tickets_earned: usize = self.tickets_earned().remove(&caller).unwrap_or_default();
            self.tickets_earned().insert(caller.clone(), tickets_earned + 1);
        } else {
            for (i, reward) in rewards.iter().enumerate() {
                if reward > 0 {
                    let mapper = self.get_token_mapper(i);
                    mapper.mint_and_send(&caller, BigUint::from(reward));
                }
            }
        }

        self.ongoing_quests(&caller).swap_remove(index_to_remove);

        if self.ongoing_quests(&caller).len() == 0 {
            self.active_players().swap_remove(&caller);
        }

        let current_battle_id = self.battles_count().get();

        self.completed_quests(current_battle_id, &caller).update(|i| {
            *i += 1;
        });
    }
}
