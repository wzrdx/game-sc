multiversx_sc::imports!();

use crate::interface::*;
use crate::{helpers, storage};

use core::iter::FromIterator;

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
    #[endpoint(setQuestsXp)]
    fn set_quests_xp(&self, values: ManagedVec<usize>) {
        self.quests_xp().clear();

        for value in values.iter() {
            self.quests_xp().push(&value);
        }
    }

    #[only_owner]
    #[endpoint(setDoubleXpTimestamp)]
    fn set_double_xp_timestamp(&self, date: u64) {
        self.double_xp_timestamp().set(date);
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

        // Payment checking & burning of tokens
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers().clone_value();
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
    #[payable("*")]
    #[endpoint(startQuests)]
    fn start_quests(&self, ids: ManagedVec<u8>) {
        self.require_conditions();
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        // Ongoing quests checking
        let ongoing_quests_ids: ManagedVec<u8> =
            ManagedVec::from_iter(self.ongoing_quests(&caller).iter().map(|q| q.id));

        for id in ids.into_iter() {
            require!(
                !ongoing_quests_ids.contains(&id),
                "Cannot start an already ongoing quest"
            );
        }

        // Quests duration checking
        let quests: ManagedVec<Quest<Self::Api>> = ManagedVec::from_iter(self.quests().iter().filter(|q| {
            let id = q.id;
            ids.contains(&id)
        }));

        // Payment checking & burning of tokens
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers().clone_value();
        let mut total_requirements: ManagedVec<u64> = ManagedVec::new();

        for quest in quests.into_iter() {
            let requirements = &quest.requirements;

            if requirements.len() > total_requirements.len() {
                let difference: usize = requirements.len() - total_requirements.len();

                for _i in 0..difference {
                    total_requirements.push(0);
                }
            }

            for (i, amount) in requirements.iter().enumerate() {
                if amount > 0 {
                    let value = total_requirements.get(i);
                    total_requirements.set(i, &(value + amount)).unwrap();
                }
            }
        }

        require!(
            payments.len() == total_requirements.iter().filter(|&x| x > 0).count(),
            "Received incorrect number of payments",
        );

        let mut payment_index: usize = 0;

        for (i, requirement) in total_requirements.iter().enumerate() {
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
        for quest in quests.into_iter() {
            self.ongoing_quests(&caller).push(&OngoingQuest {
                id: quest.id,
                end_timestamp: current_timestamp + (quest.duration as u64),
            });
        }

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

        require!(
            current_timestamp >= ongoing_quest.end_timestamp,
            "Quest cannot be completed yet"
        );

        // Rewards
        let quest: Quest<Self::Api> = self.quests().get(id as usize);
        let rewards = &quest.rewards;

        // Ticket
        if quest.is_final {
            let tickets_amount: u64 = rewards.iter().sum();

            // Stats
            self.tickets_earned(&caller).update(|n| {
                *n += tickets_amount as usize;
            });

            self.tickets_mapper()
                .nft_add_quantity_and_send(&caller, 1 as u64, BigUint::from(tickets_amount));
        } else {
            // Stats
            self.quests_type_stats(&caller, quest.quest_type).update(|n| {
                *n += 1;
            });

            self.completed_quests(&caller).update(|n| {
                *n += 1;
            });

            for (i, reward) in rewards.iter().enumerate() {
                if reward > 0 {
                    let mapper = self.get_token_mapper(i);
                    mapper.mint_and_send(&caller, BigUint::from(reward));
                }
            }
        }

        let xp_multiplier: usize = if current_timestamp <= self.double_xp_timestamp().get() {
            2
        } else {
            1
        };

        // XP
        self.player_xp(&caller).update(|i| {
            *i += xp_multiplier * self.quests_xp().get(id as usize);
        });

        self.ongoing_quests(&caller).swap_remove(index_to_remove);

        if self.ongoing_quests(&caller).len() == 0 {
            self.active_players().swap_remove(&caller);
        }
    }

    #[only_user_account]
    #[endpoint(completeAllQuests)]
    fn complete_all_quests(&self) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        let completed_quests: ManagedVec<OngoingQuest> = ManagedVec::from_iter(
            self.ongoing_quests(&caller)
                .iter()
                .filter(|q| current_timestamp >= q.end_timestamp),
        );

        let completed_quests_ids: ManagedVec<u8> = ManagedVec::from_iter(completed_quests.iter().map(|q| q.id));

        // Completed Quests
        let quests: ManagedVec<Quest<Self::Api>> = ManagedVec::from_iter(self.quests().iter().filter(|q| {
            let id = q.id;
            completed_quests_ids.contains(&id)
        }));

        // Stats
        self.completed_quests(&caller).update(|n| {
            *n += completed_quests_ids.len();
        });

        for quest_type in [1u8, 2, 3].iter() {
            let amount = quests.iter().filter(|q| q.quest_type == *quest_type).count();
            self.quests_type_stats(&caller, *quest_type).update(|n| {
                *n += amount;
            });
        }

        // Compute total rewards
        let mut total_rewards: ManagedVec<u64> = ManagedVec::new();
        let mut total_tickets_amount: u64 = 0;

        for quest in quests.into_iter() {
            let rewards = &quest.rewards;

            if quest.is_final {
                let tickets_amount: u64 = rewards.iter().sum();
                total_tickets_amount += tickets_amount;
            } else {
                if rewards.len() > total_rewards.len() {
                    let difference: usize = rewards.len() - total_rewards.len();

                    for _i in 0..difference {
                        total_rewards.push(0);
                    }
                }

                for (i, amount) in rewards.iter().enumerate() {
                    if amount > 0 {
                        let value = total_rewards.get(i);
                        total_rewards.set(i, &(value + amount)).unwrap();
                    }
                }
            }
        }

        // Send rewards
        if total_tickets_amount > 0 {
            // Stats
            self.tickets_earned(&caller).update(|n| {
                *n += total_tickets_amount as usize;
            });

            self.tickets_mapper()
                .nft_add_quantity_and_send(&caller, 1 as u64, BigUint::from(total_tickets_amount));
        }

        for (i, reward) in total_rewards.iter().enumerate() {
            if reward > 0 {
                let mapper = self.get_token_mapper(i);
                mapper.mint_and_send(&caller, BigUint::from(reward));
            }
        }

        let xp_multiplier: usize = if current_timestamp <= self.double_xp_timestamp().get() {
            2
        } else {
            1
        };

        // XP
        let xp_gain: usize = completed_quests_ids
            .iter()
            .map(|id| xp_multiplier * self.quests_xp().get(id as usize))
            .sum();

        self.player_xp(&caller).update(|i| {
            *i += xp_gain;
        });

        // Set remaining quests
        let remaining_quests: ManagedVec<OngoingQuest> = ManagedVec::from_iter(
            self.ongoing_quests(&caller)
                .iter()
                .filter(|q| current_timestamp < q.end_timestamp),
        );

        self.ongoing_quests(&caller).clear();

        for quest in remaining_quests.into_iter() {
            self.ongoing_quests(&caller).push(&quest);
        }

        if self.ongoing_quests(&caller).len() == 0 {
            self.active_players().swap_remove(&caller);
        }
    }
}
