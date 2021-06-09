use crate::*;
use failure::Fail;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fmt::Display;

#[derive(Debug, Clone, Fail)]
pub struct TransactionNotFound(pub Identifier);

impl Display for TransactionNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "transaction {} not found", self.0)
    }
}

/// A transaction store
pub trait Store {
    /// Get a transaction of an unknown type
    fn get_transaction(&self, id: Identifier) -> Option<SignedTransaction>;

    fn range(&self, start: Identifier, end_exclusive: Identifier) -> Vec<SignedTransaction>;

    fn get_multiple(
        &self,
        election_id: Identifier,
        tx_type: TransactionType,
    ) -> Vec<SignedTransaction> {
        let mut start = election_id.clone();
        start.transaction_type = tx_type;

        let mut end = start.clone();

        let mut end_type: u8 = tx_type.into();
        end_type += 1;
        match end_type.try_into() {
            Ok(tx_type) => end.transaction_type = tx_type,
            Err(_) => {
                // Wrap around
                end.transaction_type = TransactionType::Election;
                end.unique_id = Some([0; 16]);
            }
        };

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
                _ => Err(TransactionNotFound(id)),
            },
            None => Err(TransactionNotFound(id)),
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
                _ => Err(TransactionNotFound(id)),
            },
            None => Err(TransactionNotFound(id)),
        }
    }

    /// Get an Vote transaction
    fn get_vote(&self, id: Identifier) -> Result<Signed<VoteTransaction>, TransactionNotFound> {
        let tx = self.get_transaction(id);
        match tx {
            Some(tx) => match tx {
                SignedTransaction::Vote(e) => Ok(e),
                _ => Err(TransactionNotFound(id)),
            },
            None => Err(TransactionNotFound(id)),
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
                _ => Err(TransactionNotFound(id)),
            },
            None => Err(TransactionNotFound(id)),
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
                _ => Err(TransactionNotFound(id)),
            },
            None => Err(TransactionNotFound(id)),
        }
    }
}

/// A simple store that uses an in-memory BTreeMap
#[derive(Default, Clone)]
pub struct MemStore {
    inner: BTreeMap<String, SignedTransaction>,
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

    fn range(&self, start: Identifier, end_exclusive: Identifier) -> Vec<SignedTransaction> {
        let mut results = Vec::new();

        let start = start.to_string();
        let end = end_exclusive.to_string();

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

        for (_, v) in self.inner.range(start..end) {
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
