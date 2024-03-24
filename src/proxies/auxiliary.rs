multiversx_sc::imports!();

#[multiversx_sc::proxy]
pub trait AuxiliaryContract {
    #[view(getPlayerInfo)]
    fn get_player_info(&self, user: &ManagedAddress) -> (usize, BigUint<Self::Api>);
}
