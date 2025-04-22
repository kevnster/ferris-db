mod db; 
use db::{InMemoryDB};

fn main() {
    let mut db = InMemoryDB::new();

    // inmemoryDB.get("A")
    println!("inmemoryDB.get(\"A\"): {:?}", db.get("A"));

    // inmemoryDB.put(“A”, 5);
    match db.put("A".to_string(), 5) {
        Ok(()) => println!("Success (unexpected!)"), 
        Err(e) => println!("Failed as expected: {}", e), 
    }

    // inmemoryDB.begin_transaction();
    match db.begin_transaction() {
        Ok(()) => {
            println!("Transaction started successfully.");

            // inmemoryDB.put(“A”, 5);
            match db.put("A".to_string(), 5) {
                Ok(()) => println!("Put successful."),
                Err(e) => println!("Put failed unexpectedly: {}", e),
            }

            // inmemoryDB.get(“A”);
            println!("inmemoryDB.get(\"A\"): {:?}", db.get("A"));

            // inmemoryDB.put(“A”, 6);
            match db.put("A".to_string(), 6) {
                Ok(()) => println!("Put successful."),
                Err(e) => println!("Put failed unexpectedly: {}", e),
            }

            // inmemoryDB.commit();
            match db.commit() {
                Ok(()) => println!("Commit successful."),
                Err(e) => println!("Commit failed unexpectedly: {}", e),
            }
        }
        Err(e) => println!("Failed to begin transaction: {}", e),
    }

    // inmemoryDB.get(“A”);
    println!("inmemoryDB.get(\"A\"): {:?}", db.get("A"));

    // inmemoryDB.commit();
    match db.commit() {
        Ok(()) => println!("Success (unexpected!)"),
        Err(e) => println!("Failed as expected: {}", e),
    }

    // inmemoryDB.rollback();
    match db.rollback() {
        Ok(()) => println!("Success (unexpected!)"),
        Err(e) => println!("Failed as expected: {}", e),
    }

    // inmemoryDB.get(“B”);
    println!("inmemoryDB.get(\"B\"): {:?}", db.get("B"));

    // inmemoryDB.begin_transaction();
    match db.begin_transaction() {
        Ok(()) => {
            println!("Transaction 2 started successfully.");

            // inmemoryDB.put("B", 10); 
            match db.put("B".to_string(), 10) {
                Ok(()) => println!("Put successful."),
                Err(e) => println!("Put failed unexpectedly: {}", e),
            }

            // inmemoryDB.rollback();
            println!("inmemoryDB.rollback();");
            match db.rollback() {
                 Ok(()) => println!("Rollback successful."),
                 Err(e) => println!("Rollback failed unexpectedly: {}", e),
            }
        }
        Err(e) => println!("Failed to begin transaction 2: {}", e),
    }

    // inmemoryDB.get(“B”)
    println!("inmemoryDB.get(\"B\"): {:?}", db.get("B"));
}
