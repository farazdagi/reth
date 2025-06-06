use crate::blobstore::{BlobStore, BlobStoreCleanupStat, BlobStoreError};
use alloy_eips::eip4844::{BlobAndProofV1, BlobAndProofV2, BlobTransactionSidecar};
use alloy_primitives::B256;
use std::sync::Arc;

/// A blobstore implementation that does nothing
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct NoopBlobStore;

impl BlobStore for NoopBlobStore {
    fn insert(&self, _tx: B256, _data: BlobTransactionSidecar) -> Result<(), BlobStoreError> {
        Ok(())
    }

    fn insert_all(&self, _txs: Vec<(B256, BlobTransactionSidecar)>) -> Result<(), BlobStoreError> {
        Ok(())
    }

    fn delete(&self, _tx: B256) -> Result<(), BlobStoreError> {
        Ok(())
    }

    fn delete_all(&self, _txs: Vec<B256>) -> Result<(), BlobStoreError> {
        Ok(())
    }

    fn cleanup(&self) -> BlobStoreCleanupStat {
        BlobStoreCleanupStat::default()
    }

    fn get(&self, _tx: B256) -> Result<Option<Arc<BlobTransactionSidecar>>, BlobStoreError> {
        Ok(None)
    }

    fn contains(&self, _tx: B256) -> Result<bool, BlobStoreError> {
        Ok(false)
    }

    fn get_all(
        &self,
        _txs: Vec<B256>,
    ) -> Result<Vec<(B256, Arc<BlobTransactionSidecar>)>, BlobStoreError> {
        Ok(vec![])
    }

    fn get_exact(
        &self,
        txs: Vec<B256>,
    ) -> Result<Vec<Arc<BlobTransactionSidecar>>, BlobStoreError> {
        if txs.is_empty() {
            return Ok(vec![])
        }
        Err(BlobStoreError::MissingSidecar(txs[0]))
    }

    fn get_by_versioned_hashes_v1(
        &self,
        versioned_hashes: &[B256],
    ) -> Result<Vec<Option<BlobAndProofV1>>, BlobStoreError> {
        Ok(vec![None; versioned_hashes.len()])
    }

    fn get_by_versioned_hashes_v2(
        &self,
        _versioned_hashes: &[B256],
    ) -> Result<Option<Vec<BlobAndProofV2>>, BlobStoreError> {
        Ok(None)
    }

    fn data_size_hint(&self) -> Option<usize> {
        Some(0)
    }

    fn blobs_len(&self) -> usize {
        0
    }
}
