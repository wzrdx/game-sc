multiversx_sc::imports!();

#[multiversx_sc::proxy]
pub trait AuxiliaryContract {
    #[view(getPagesMinted)]
    fn get_pages_minted(&self, user: &ManagedAddress) -> usize;
}
