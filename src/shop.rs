multiversx_sc::imports!();

use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Shop:
    multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule + storage::Storage + helpers::Helpers
{
}
