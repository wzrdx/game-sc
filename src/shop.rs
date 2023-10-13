multiversx_sc::imports!();

use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Shop: storage::Storage + helpers::Helpers {
    #[only_owner]
    #[endpoint(debug)]
    fn debug(&self) {}

    #[only_user_account]
    #[payable("*")]
    #[endpoint(stub)]
    fn stub(&self) {}
}
