multiversx_sc::imports!();

#[multiversx_sc::proxy]
pub trait AuxiliaryContract {
    #[endpoint(add)]
    fn add(&self, arg: usize);
}
