use reth_primitives::BlockNumber;

/// Determines the control flow during pipeline execution.
#[derive(Debug)]
pub(crate) enum ControlFlow {
    /// An unwind was requested and must be performed before continuing.
    Unwind {
        /// The block to unwind to.
        target: BlockNumber,
        /// The block that caused the unwind.
        bad_block: Option<BlockNumber>,
    },
    /// The pipeline is allowed to continue executing stages.
    Continue,
}