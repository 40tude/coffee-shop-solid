// SOLID: JsonOrderRepository - JSON file storage adapter
// 
// PRINCIPLES DEMONSTRATED:
// 
// 1. OPEN-CLOSED PRINCIPLE (OCP):
//    This is a NEW storage implementation added WITHOUT modifying:
//    - OrderRepository trait (it was closed)
//    - OrderService (it depends on the trait)
//    - MemoryOrderRepository (it's independent)
//    - Any other existing code
// 
// 2. LISKOV SUBSTITUTION PRINCIPLE (LSP):
//    This must be substitutable for MemoryOrderRepository
//    OrderService should work identically with either implementation
// 
// 3. DEPENDENCY INVERSION PRINCIPLE (DIP):
//    This adapter depends on the OrderRepository trait (abstraction)
//    It implements the interface defined by the high-level layer

use crate::domain::Order;
use crate::ports::{OrderRepository, RepositoryError};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

/// JSON file-based order repository
/// 
/// SOLID PRINCIPLE: Open-Closed Principle (OCP)
/// 
/// We added this implementation WITHOUT modifying any existing code:
/// - OrderRepository trait was defined once and is now CLOSED
/// - OrderService is OPEN to using this new implementation
/// - We just "plug in" this adapter
/// 
/// SOLID PRINCIPLE: Liskov Substitution Principle (LSP)
/// 
/// This implementation MUST be substitutable for MemoryOrderRepository.
/// From OrderService's perspective, it shouldn't matter if we use Memory or JSON.
/// The behavior must be consistent.
/// 
/// USE CASE:
/// Perfect for:
/// - Persistence between runs (data survives restarts)
/// - Simple deployments (no database server needed)
/// - Demos where you want to inspect the data (JSON is human-readable)
/// - Development (easy to debug - just look at the JSON file)
pub struct JsonOrderRepository {
    file_path: PathBuf,
    orders: HashMap<Uuid, Order>,
}

impl JsonOrderRepository {
    /// Create a new JSON repository
    /// 
    /// The repository will store orders in a JSON file at the given path.
    /// If the file exists, orders are loaded. If not, starts empty.
    pub fn new(file_path: PathBuf) -> Result<Self, RepositoryError> {
        let orders = if file_path.exists() {
            // Load existing orders
            Self::load_from_file(&file_path)?
        } else {
            // Start with empty repository
            HashMap::new()
        };

        Ok(Self { file_path, orders })
    }

    /// Load orders from JSON file
    fn load_from_file(path: &PathBuf) -> Result<HashMap<Uuid, Order>, RepositoryError> {
        let contents = fs::read_to_string(path).map_err(|e| {
            RepositoryError::LoadFailed(format!("Failed to read file: {}", e))
        })?;

        let orders: Vec<Order> = serde_json::from_str(&contents).map_err(|e| {
            RepositoryError::LoadFailed(format!("Failed to parse JSON: {}", e))
        })?;

        let mut map = HashMap::new();
        for order in orders {
            map.insert(order.id, order);
        }

        Ok(map)
    }

    /// Save orders to JSON file
    fn save_to_file(&self) -> Result<(), RepositoryError> {
        let orders: Vec<&Order> = self.orders.values().collect();

        let json = serde_json::to_string_pretty(&orders).map_err(|e| {
            RepositoryError::SaveFailed(format!("Failed to serialize orders: {}", e))
        })?;

        fs::write(&self.file_path, json).map_err(|e| {
            RepositoryError::SaveFailed(format!("Failed to write file: {}", e))
        })?;

        Ok(())
    }
}

/// SOLID PRINCIPLE: Liskov Substitution Principle (LSP)
/// 
/// This implementation MUST behave exactly like MemoryOrderRepository
/// from the caller's perspective. Same contract, same semantics.
/// 
/// The only difference is WHERE the data is stored (memory vs file),
/// not HOW the interface behaves.
/// 
/// SOLID PRINCIPLE: Open-Closed Principle (OCP)
/// 
/// Adding this implementation required:
/// - Creating this file
/// - Implementing the OrderRepository trait
/// 
/// It did NOT require:
/// - Modifying OrderService
/// - Modifying OrderRepository trait
/// - Modifying MemoryOrderRepository
/// - Modifying any tests
/// 
/// The system was OPEN for this extension but CLOSED for modification.
impl OrderRepository for JsonOrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), RepositoryError> {
        // LSP: Honor the same contract as MemoryOrderRepository
        if self.orders.contains_key(&order.id) {
            return Err(RepositoryError::AlreadyExists(format!(
                "Order {} already exists",
                order.id
            )));
        }

        self.orders.insert(order.id, order.clone());

        // Persist to file after every save
        // (In a real system, you might batch writes for performance)
        self.save_to_file()?;

        Ok(())
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Order>, RepositoryError> {
        // LSP: Exact same behavior as MemoryOrderRepository
        Ok(self.orders.get(&id).cloned())
    }

    fn find_by_customer_email(&self, email: &str) -> Result<Vec<Order>, RepositoryError> {
        // LSP: Exact same behavior as MemoryOrderRepository
        let orders: Vec<Order> = self
            .orders
            .values()
            .filter(|order| order.customer.email == email)
            .cloned()
            .collect();

        Ok(orders)
    }

    fn list_all(&self) -> Result<Vec<Order>, RepositoryError> {
        // LSP: Exact same behavior as MemoryOrderRepository
        Ok(self.orders.values().cloned().collect())
    }

    fn update(&mut self, order: &Order) -> Result<(), RepositoryError> {
        // LSP: Exact same contract - return NotFound if doesn't exist
        if !self.orders.contains_key(&order.id) {
            return Err(RepositoryError::NotFound(format!(
                "Order {} not found",
                order.id
            )));
        }

        self.orders.insert(order.id, order.clone());

        // Persist to file
        self.save_to_file()?;

        Ok(())
    }

    fn delete(&mut self, id: Uuid) -> Result<bool, RepositoryError> {
        // LSP: Exact same contract - return true if existed, false if didn't
        let existed = self.orders.remove(&id).is_some();

        if existed {
            // Persist to file
            self.save_to_file()?;
        }

        Ok(existed)
    }
}

// ============================================================================
// OCP IN ACTION: Swapping Implementations
// 
// In main.rs, we can easily switch between implementations:
// 
// // Use in-memory storage (fast, no persistence)
// let repo = MemoryOrderRepository::new();
// let service = OrderService::new(repo, payment, notifier);
// 
// // Use JSON storage (persistent, human-readable)
// let repo = JsonOrderRepository::new("orders.json".into())?;
// let service = OrderService::new(repo, payment, notifier);
// 
// // Future: Use PostgreSQL (scalable, transactional)
// let repo = PostgresOrderRepository::new(connection_string)?;
// let service = OrderService::new(repo, payment, notifier);
// 
// OrderService doesn't change. Business logic doesn't change.
// We just plug in different adapters. That's OCP + DIP working together!
// ============================================================================

// ============================================================================
// LSP IN ACTION: Behavioral Consistency
// 
// This code should work identically with Memory OR JSON storage:
// 
// fn test_order_workflow<R: OrderRepository>(mut repo: R) {
//     let order = create_test_order();
//     
//     repo.save(&order).unwrap();
//     let found = repo.find_by_id(order.id).unwrap();
//     assert!(found.is_some());
//     
//     repo.delete(order.id).unwrap();
//     let found = repo.find_by_id(order.id).unwrap();
//     assert!(found.is_none());
// }
// 
// test_order_workflow(MemoryOrderRepository::new());
// test_order_workflow(JsonOrderRepository::new("test.json".into()).unwrap());
// 
// Both should pass! That's LSP ensuring substitutability.
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Customer, OrderItem};
    use std::env;

    fn make_test_order() -> Order {
        let customer = Customer::new(
            "Test User".to_string(),
            "test@example.com".to_string(),
            None,
        );

        let items = vec![OrderItem {
            beverage_name: "Coffee".to_string(),
            beverage_description: "Medium Coffee".to_string(),
            price: 3.50,
            quantity: 1,
        }];

        Order::new(customer, items)
    }

    #[test]
    fn test_save_and_load() {
        let temp_file = env::temp_dir().join("test_orders.json");

        // Create repo and save order
        {
            let mut repo = JsonOrderRepository::new(temp_file.clone()).unwrap();
            let order = make_test_order();
            repo.save(&order).unwrap();
        }

        // Load from file - order should persist
        {
            let repo = JsonOrderRepository::new(temp_file.clone()).unwrap();
            let orders = repo.list_all().unwrap();
            assert_eq!(orders.len(), 1);
        }

        // Cleanup
        let _ = fs::remove_file(temp_file);
    }

    #[test]
    fn test_lsp_consistency() {
        // This test verifies LSP: JsonOrderRepository should behave
        // exactly like MemoryOrderRepository

        let temp_file = env::temp_dir().join("test_lsp.json");
        let mut repo = JsonOrderRepository::new(temp_file.clone()).unwrap();

        let order = make_test_order();

        // Test save
        repo.save(&order).unwrap();

        // Test find
        let found = repo.find_by_id(order.id).unwrap();
        assert!(found.is_some());

        // Test duplicate save fails (LSP contract)
        let result = repo.save(&order);
        assert!(result.is_err());

        // Test delete
        let deleted = repo.delete(order.id).unwrap();
        assert!(deleted);

        // Second delete returns false (LSP contract)
        let deleted = repo.delete(order.id).unwrap();
        assert!(!deleted);

        // Cleanup
        let _ = fs::remove_file(temp_file);
    }
}
