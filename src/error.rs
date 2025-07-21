use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    OutOfBoundsUpdate { index: usize, len: u64 },
    OutOfBoundsIterFrom { index: usize, len: usize },
    ListFull { len: usize },
    PackedLeafFull { len: usize },
    LeafUpdateMissing { index: usize },
    PackedLeafOutOfBounds { sub_index: usize, len: usize },
    NodeUpdatesMissing { prefix: usize },
    InvalidListUpdate,
    InvalidVectorUpdate,
    WrongVectorLength { len: usize, expected: usize },
    PushNotSupported,
    UpdateLeafError,
    UpdateLeavesError,
    InvalidRebaseNode,
    InvalidRebaseLeaf,
    BuilderInvalidDepth { depth: usize },
    BuilderExpectedLeaf,
    BuilderStackEmptyMerge,
    BuilderStackEmptyMergeLeft,
    BuilderStackEmptyMergeRight,
    BuilderStackEmptyFinish,
    BuilderStackEmptyFinishLeft,
    BuilderStackEmptyFinishRight,
    BuilderStackEmptyFinalize,
    BuilderStackLeftover,
    BuilderFull,
    BulkUpdateUnclean,
    CowMissingEntry,
    LevelIterPendingUpdates,
    IntraRebaseZeroHash,
    IntraRebaseZeroDepth,
    IntraRebaseRepeatVisit,
    LengthTooLarge { length: u64, max_length: u64 },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{self:?}")
    }
}
