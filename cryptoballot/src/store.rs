use crate::*;
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error("{tx_type} transaction {id} not found")]
pub struct TransactionNotFound {
    pub id: Identifier,
    pub tx_type: TransactionType,
}

impl TransactionNotFound {
    fn new(id: Identifier, tx_type: TransactionType) -> Self {
        Self { id, tx_type }
    }
}

/// A transaction store
pub trait Store {
    /// Get a transaction of an unknown type
    fn get_transaction(&self, id: Identifier) -> Option<SignedTransaction>;

    fn range(&self, start: Identifier, end_inclusive: Identifier) -> Vec<SignedTransaction>;

    fn get_multiple(
        &self,
        election_id: Identifier,
        tx_type: TransactionType,
    ) -> Vec<SignedTransaction> {
        let start = Identifier::start(election_id, tx_type, None);
        let end = Identifier::end(election_id, tx_type, None);

        self.range(start, end)
    }

    // TODO: Macro these methods

    /// Get an election transaction
    fn get_election(
        &self,
        id: Identifier,
    ) -> Result<Signed<ElectionTransaction>, TransactionNotFound> {
        let tx = self.get_transaction(id);
        match tx {
            Some(tx) => match tx {
                SignedTransaction::Election(e) => Ok(e),
                _ => Err(TransactionNotFound::new(id, TransactionType::Election)),
            },
            None => Err(TransactionNotFound::new(id, TransactionType::Election)),
        }
    }

    /// Get a public_key transaction
    fn get_keygen_public_key(
        &self,
        id: Identifier,
    ) -> Result<Signed<KeyGenPublicKeyTransaction>, TransactionNotFound> {
        let tx = self.get_transaction(id);
        match tx {
            Some(tx) => match tx {
                SignedTransaction::KeyGenPublicKey(e) => Ok(e),
                _ => Err(TransactionNotFound::new(
                    id,
                    TransactionType::KeyGenPublicKey,
                )),
            },
            None => Err(TransactionNotFound::new(
                id,
                TransactionType::KeyGenPublicKey,
            )),
        }
    }

    /// Get an Vote transaction
    fn get_vote(&self, id: Identifier) -> Result<Signed<VoteTransaction>, TransactionNotFound> {
        let tx = self.get_transaction(id);
        match tx {
            Some(tx) => match tx {
                SignedTransaction::Vote(e) => Ok(e),
                _ => Err(TransactionNotFound::new(id, TransactionType::Vote)),
            },
            None => Err(TransactionNotFound::new(id, TransactionType::Vote)),
        }
    }

    /// Get an Mix transaction
    fn get_mix(&self, id: Identifier) -> Result<Signed<MixTransaction>, TransactionNotFound> {
        let tx = self.get_transaction(id);
        match tx {
            Some(tx) => match tx {
                SignedTransaction::Mix(e) => Ok(e),
                _ => Err(TransactionNotFound::new(id, TransactionType::Mix)),
            },
            None => Err(TransactionNotFound::new(id, TransactionType::Mix)),
        }
    }

    /// Get a PartialDecryption transaction
    fn get_partial_decryption(
        &self,
        id: Identifier,
    ) -> Result<Signed<PartialDecryptionTransaction>, TransactionNotFound> {
        let tx = self.get_transaction(id);
        match tx {
            Some(tx) => match tx {
                SignedTransaction::PartialDecryption(e) => Ok(e),
                _ => Err(TransactionNotFound::new(
                    id,
                    TransactionType::PartialDecryption,
                )),
            },
            None => Err(TransactionNotFound::new(
                id,
                TransactionType::PartialDecryption,
            )),
        }
    }

    /// Get a Decryption transaction
    fn get_decryption(
        &self,
        id: Identifier,
    ) -> Result<Signed<DecryptionTransaction>, TransactionNotFound> {
        let tx = self.get_transaction(id);
        match tx {
            Some(tx) => match tx {
                SignedTransaction::Decryption(e) => Ok(e),
                _ => Err(TransactionNotFound::new(id, TransactionType::Decryption)),
            },
            None => Err(TransactionNotFound::new(id, TransactionType::Decryption)),
        }
    }
}

/// A simple store that uses an in-memory BTreeMap
#[derive(Default, Clone)]
pub struct MemStore {
    pub(crate) inner: BTreeMap<String, SignedTransaction>,
}

impl MemStore {
    pub fn set(&mut self, tx: SignedTransaction) {
        self.inner.insert(tx.id().to_string(), tx);
    }
}

impl Store for MemStore {
    fn get_transaction(&self, id: Identifier) -> Option<SignedTransaction> {
        let key = id.to_string();
        self.inner.get(&key).cloned()
    }

    fn range(&self, start: Identifier, end_inclusive: Identifier) -> Vec<SignedTransaction> {
        let mut results = Vec::new();

        let start = start.to_string();
        let end = end_inclusive.to_string();

        // TODO: Go back to using array keys (faster)
        // OLD CODE For Array Keys:
        //let election_id = election_id.to_array();
        //let mut start: [u8; 32] = [0; 32];
        //start[..15].copy_from_slice(&election_id[..15]);
        //start[16] = tx_type as u8;

        // End at the next transaction type
        //let mut end: [u8; 32] = [0; 32];
        //end[..15].copy_from_slice(&election_id[..15]);
        //end[16] = (tx_type as u8) + 1;

        for (_, v) in self.inner.range(start..=end) {
            results.push(v.clone())
        }
        results
    }
}

impl From<Vec<SignedTransaction>> for MemStore {
    fn from(item: Vec<SignedTransaction>) -> Self {
        let mut memstore = MemStore::default();
        for tx in item {
            memstore.set(tx);
        }
        memstore
    }
}
