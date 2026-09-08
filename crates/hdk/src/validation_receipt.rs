use crate::hdk::HDK;
use hdi::map_extern::ExternResult;
use holochain_zome_types::prelude::{GetValidationReceiptsInput, ValidationReceiptSet};

/// Get validation receipts associated with an action.
///
/// When an action is created, it is represented as multiple DHT ops to be published on the network.
/// Each op will be validated by other agents on the network and a receipt will be sent back to the
/// author. The return value of this function is organized by DHT op hash. Each [ValidationReceiptSet]
/// contains all the validation receipts that have been received for that DHT op.
///
/// Note: This function will permit you to look for validation receipts for any action hash, but you
/// will only have receipts if the action was authored on the same conductor. Not necessarily the
/// same agent, but it must be the same conductor.
///
/// ### Example
/// ```rust,no_run
/// use hdk::prelude::*;
///
/// # fn main() -> ExternResult<()> {
///     // Use the action hash returned when the entry was created.
///     let action_hash = ActionHash::from_raw_36(vec![0; 36]);
///     let receipts = get_validation_receipts(GetValidationReceiptsInput::new(action_hash))?;
///     let count = receipts
///         .into_iter()
///         .filter(|receipt_set| receipt_set.op_type == "AgentActivity")
///         .flat_map(|receipt_set| receipt_set.receipts)
///         .count();
///     info!("Found {} receipts from agent activity authorities", count);
/// #     Ok(())
/// # }
/// ```
pub fn get_validation_receipts(
    input: GetValidationReceiptsInput,
) -> ExternResult<Vec<ValidationReceiptSet>> {
    HDK.with(|h| h.borrow().get_validation_receipts(input))
}
