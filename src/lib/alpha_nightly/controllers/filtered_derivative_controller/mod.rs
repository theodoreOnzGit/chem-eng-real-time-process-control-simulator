use crate::alpha_nightly::transfer_fn_wrapper_and_enums::TransferFnFirstOrder;

/// a filtered derivative controller 
///
/// G(s) = s / (0.1 s + 1)
///
/// The form is identical to that of a first order transfer function 
/// with s = 0 as its only zero
///
/// Therefore, I'll just have this struct house a transfer function
///
///
pub struct FilteredDerivativeController{
    pub transfer_fn: TransferFnFirstOrder,
}


