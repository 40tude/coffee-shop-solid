// SOLID: MemoryOrderRepository - In-memory storage adapter
// 
// PRINCIPLES DEMONSTRATED:
// 
// 1. DEPENDENCY INVERSION PRINCIPLE (DIP):
//    This adapter depends on the OrderRepository TRAIT (abstraction)
//    It doesn't define its own interface; it implements the interface
//    defined by the high-level ports layer
// 
// 2. LISKOV SUBSTITUTION PRINCIPLE (LSP):
//    This implementation must honor the OrderRepository contract
//    It must be substitutable with any other OrderRepository implementation
//    (JsonOrderRepository, PostgresOrderRepository, etc.)
// 
// 3. OPEN-CLOSED PRINCIPLE (OCP):
//    We added this implementation without modifying OrderService or any other code
//    The system was open for this extension

use crate::domain::Order;
use crate::ports::{OrderRepository, RepositoryError};
use std::collections::HashMap;
use uuid::Uuid;

/// In-memory order repository
/// 
/// SOLID PRINCIPLE: Liskov Substitution Principle (LSP)
/// 
/// This implementation MUST honor the OrderRepository contract:
/// - Same method signatures
/// - Same error semantics
/// - Same behavior guarantees
/// 
/// A caller using OrderRepository shouldn't care if it's Memory, JSON, or Postgres.
/// They should all behave consistently.
/// 
/// USE CASE:
/// Perfect for:
/// - Unit testing (no file I/O or database needed)
/// - Development (fast, no setup required)
/// - Demos (no persistence between runs)
pub struct MemoryOrderRepository {
    orders: HashMap<Uuid, Order>,
}

impl MemoryOrderRepository {
    /// Create a new in-memory repository
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
        }
    }

    /// Get the number of orders (useful for testing)
    pub fn count(&self) -> usize {
        self.orders.len()
    }

    /// Clear all orders (useful for testing)
    pub fn clear(&mut self) {
        self.orders.clear();
    }
}

impl Default for MemoryOrderRepository {
    fn default() -> Self {
        Self::new()
    }
}

/// SOLID PRINCIPLE: Liskov Substitution Principle (LSP)
/// 
/// This implementation MUST be substitutable for any other OrderRepository.
/// We must honor the contract defined by the trait:
/// - Return types must match
/// - Error semantics must be consistent
/// - Behavior must be predictable
/// 
/// SOLID PRINCIPLE: Open-Closed Principle (OCP)
/// 
/// Notice: We added this implementation without modifying:
/// - OrderRepository trait (it was closed)
/// - OrderService (it depends on the trait, not this implementation)
/// - Any other existing code
/// 
/// This is extension without modification - OCP in action.
impl OrderRepository for MemoryOrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), RepositoryError> {
        // LSP CONTRACT: If order.id already exists, return AlreadyExists error
        if self.orders.contains_key(&order.id) {
            return Err(RepositoryError::AlreadyExists(format!(
                "Order {} already exists",
                order.id
            )));
        }

        // Save the order
        self.orders.insert(order.id, order.clone());

        // LSP CONTRACT: Return Ok(()) on success
        Ok(())
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Order>, RepositoryError> {
        // LSP CONTRACT: Return Ok(Some(order)) if found, Ok(None) if not found
        Ok(self.orders.get(&id).cloned())
    }

    fn find_by_customer_email(&self, email: &str) -> Result<Vec<Order>, RepositoryError> {
        // LSP CONTRACT: Return all matching orders (can be empty vec)
        let orders: Vec<Order> = self
            .orders
            .values()
            .filter(|order| order.customer.email == email)
            .cloned()
            .collect();

        Ok(orders)
    }

    fn list_all(&self) -> Result<Vec<Order>, RepositoryError> {
        // LSP CONTRACT: Return all orders (can be empty vec)
        Ok(self.orders.values().cloned().collect())
    }

    fn update(&mut self, order: &Order) -> Result<(), RepositoryError> {
        // LSP CONTRACT: Return NotFound if order doesn't exist
        if !self.orders.contains_key(&order.id) {
            return Err(RepositoryError::NotFound(format!(
                "Order {} not found",
                order.id
            )));
        }

        // Update the order
        self.orders.insert(order.id, order.clone());

        // LSP CONTRACT: Return Ok(()) on success
        Ok(())
    }

    fn delete(&mut self, id: Uuid) -> Result<bool, RepositoryError> {
        // LSP CONTRACT: Return true if existed and deleted, false if didn't exist
        Ok(self.orders.remove(&id).is_some())
    }
}

// ============================================================================
// LSP IN ACTION: Why Contract Consistency Matters
// 
// Imagine if we violated LSP:
// 
// Bad implementation:
// impl OrderRepository for BadMemoryRepo {
//     fn save(&mut self, order: &Order) -> Result<(), RepositoryError> {
//         self.orders.insert(order.id, order.clone());
//         Ok(())  // BUG: Doesn't check for duplicates!
//     }
// }
// 
// This violates LSP because:
// 1. The contract says to return AlreadyExists on duplicates
// 2. This implementation silently overwrites
// 3. Code depending on OrderRepository would break
// 
// Example breakage:
// fn process_order(repo: &mut dyn OrderRepository, order: Order) {
//     repo.save(&order).expect("Order is new");  // CRASH with BadMemoryRepo!
// }
// 
// LSP ensures that ANY OrderRepository behaves the same way.
// Users can substitute implementations without fear of surprises.
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Customer, OrderItem, OrderStatus};

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
    fn test_save_and_find() {
        let mut repo = MemoryOrderRepository::new();
        let order = make_test_order();

        // Save
        let result = repo.save(&order);
        assert!(result.is_ok());

        // Find
        let found = repo.find_by_id(order.id).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, order.id);
    }

    #[test]
    fn test_save_duplicate_fails() {
        let mut repo = MemoryOrderRepository::new();
        let order = make_test_order();

        // First save succeeds
        repo.save(&order).unwrap();

        // Second save should fail (LSP contract)
        let result = repo.save(&order);
        assert!(result.is_err());
        assert!(matches!(result, Err(RepositoryError::AlreadyExists(_))));
    }

    #[test]
    fn test_find_by_customer_email() {
        let mut repo = MemoryOrderRepository::new();

        let customer = Customer::new(
            "Alice".to_string(),
            "alice@example.com".to_string(),
            None,
        );

        let order1 = Order::new(customer.clone(), vec![]);
        let order2 = Order::new(customer, vec![]);

        repo.save(&order1).unwrap();
        repo.save(&order2).unwrap();

        let orders = repo.find_by_customer_email("alice@example.com").unwrap();
        assert_eq!(orders.len(), 2);
    }

    #[test]
    fn test_update() {
        let mut repo = MemoryOrderRepository::new();
        let mut order = make_test_order();

        repo.save(&order).unwrap();

        // Update status
        order.mark_as_paid("PAY-123".to_string());
        repo.update(&order).unwrap();

        // Verify
        let found = repo.find_by_id(order.id).unwrap().unwrap();
        assert_eq!(found.status, OrderStatus::Paid);
    }

    #[test]
    fn test_delete() {
        let mut repo = MemoryOrderRepository::new();
        let order = make_test_order();

        repo.save(&order).unwrap();

        // Delete returns true if existed
        let deleted = repo.delete(order.id).unwrap();
        assert!(deleted);

        // Delete returns false if didn't exist
        let deleted = repo.delete(order.id).unwrap();
        assert!(!deleted);
    }

    #[test]
    fn test_list_all() {
        let mut repo = MemoryOrderRepository::new();

        let order1 = make_test_order();
        let order2 = make_test_order();

        repo.save(&order1).unwrap();
        repo.save(&order2).unwrap();

        let all = repo.list_all().unwrap();
        assert_eq!(all.len(), 2);
    }
}
