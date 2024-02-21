//! Data structures related to Merkle trees based on RPO256 hash function.

use super::{
    hash::rpo::{Rpo256, RpoDigest},
    utils::collections::{KvMap, RecordingMap},
    Felt, Word, EMPTY_WORD, ZERO,
};

// REEXPORTS
// ================================================================================================

mod empty_roots;
pub use empty_roots::EmptySubtreeRoots;

mod index;
pub use index::NodeIndex;

mod merkle_tree;
pub use merkle_tree::{path_to_text, tree_to_text, MerkleTree};

mod path;
pub use path::{MerklePath, RootPath, ValuePath};

mod smt;
pub use smt::{
    LeafIndex, SimpleSmt, Smt, SmtLeaf, SmtLeafError, SmtProof, SmtProofError, SMT_DEPTH,
    SMT_MAX_DEPTH, SMT_MIN_DEPTH,
};

mod mmr;
pub use mmr::{InOrderIndex, Mmr, MmrDelta, MmrError, MmrPeaks, MmrProof, PartialMmr};

mod store;
pub use store::{DefaultMerkleStore, MerkleStore, RecordingMerkleStore, StoreNode};

mod node;
pub use node::InnerNodeInfo;

mod partial_mt;
pub use partial_mt::PartialMerkleTree;

mod error;
pub use error::MerkleError;

// HELPER FUNCTIONS
// ================================================================================================

#[cfg(test)]
use winter_utils::collections::*;

#[cfg(test)]
const fn int_to_node(value: u64) -> RpoDigest {
    RpoDigest::new([Felt::new(value), ZERO, ZERO, ZERO])
}

#[cfg(test)]
const fn int_to_leaf(value: u64) -> Word {
    [Felt::new(value), ZERO, ZERO, ZERO]
}

#[cfg(test)]
fn digests_to_words(digests: &[RpoDigest]) -> Vec<Word> {
    digests.iter().map(|d| d.into()).collect()
}
