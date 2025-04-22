use std::collections::HashMap; // key-val storage
use std::fmt; // Display trait

#[derive(Debug)] // print enum var
pub enum DBError {
    TransactionAlreadyActive,
    NoActiveTransaction,
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DBError::TransactionAlreadyActive => write!(f, "Transaction already active"),
            DBError::NoActiveTransaction => write!(f, "No active transaction"),
        }
    }
}

// req Debug & Display
impl std::error::Error for DBError {}

// impl Default trait, aka creating a default instance using constructor
#[derive(Default)]
pub struct InMemoryDB {
    main_state: HashMap<String, i32>, // key-val pair (str, int)
    transaction_state: Option<HashMap<String, i32>>, // stores temp changes; if 'None' = no transaction active
}

impl InMemoryDB {
    pub fn new() -> Self {
        // calls #[derive(Default)] -> an InMemoryDB w/ main_state = empty HM & transaction_state = None
        Default::default()  
    }

    // mut ref to self instance to change the transaction_state
    pub fn begin_transaction(&mut self) -> Result<(), DBError> {
        // check if transaction_state is already 'Some(...)'
        if self.transaction_state.is_some() {
            // if yes, panic b/c a transaction is alr active
            return Err(DBError::TransactionAlreadyActive);
        }

        // if 'None', change it to 'Some' : a new, empty HM to start a transaction for temp storage of 'put' calls
        self.transaction_state = Some(HashMap::new());
        
        Ok(())
    }

    pub fn put(&mut self, key: String, val: i32) -> Result<(), DBError> {
        // safe way to get mutual ref
        if let Some(curr_txn_state) = self.transaction_state.as_mut() {
            // now curr_txn_state mutuable ref to HM & insert k-p into temp trans HM 
            curr_txn_state.insert(key, val);
            
            Ok(())
        } else {
            return Err(DBError::NoActiveTransaction);
        }
    }

    pub fn get(&self, key: &str) -> Option<i32> {
        // .get(key) -> Option<&i32>
        // .copied() converts Option<&i32> -> Option<i32> thru copy 
        self.main_state.get(key).copied()
    }

    pub fn commit(&mut self) -> Result<(), DBError> {
        // check if trans_state is Option -> move val out to leave None behind (end trans)
        if let Some(curr_txn_state) = self.transaction_state.take() {
            // curr_txn_state holds trans HM & loop thru k-v pairs to insert into main_state = make changes PERMANENT!
            for (key, val) in curr_txn_state {
                self.main_state.insert(key, val);
            }

            Ok(())
        } else {
            return Err(DBError::NoActiveTransaction);
        }
    }

    pub fn rollback(&mut self) -> Result<(), DBError> {
        if self.transaction_state.is_none() { 
            return Err(DBError::NoActiveTransaction);
        }

        // discards temp HM (& all changes) IF trans_state is active (e.g. is_sone is true)
        self.transaction_state = None; 
        
        Ok(())
    }
}