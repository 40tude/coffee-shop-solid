// SOLID: This module is part of the DOMAIN layer
// Domain entities are pure data structures with minimal logic
// They have NO dependencies on infrastructure (databases, HTTP, etc.)
// This follows the Dependency Inversion Principle (DIP)

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a customer in our coffee shop
/// 
/// SOLID PRINCIPLE: Single Responsibility Principle (SRP)
/// This struct has ONE responsibility: represent a customer's data
/// It doesn't know how to:
/// - Save itself to a database (that's the Repository's job)
/// - Send itself notifications (that's the Notifier's job)
/// - Calculate discounts (that's the PricingCalculator's job)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

impl Customer {
    /// Create a new customer
    /// 
    /// Note: This is just a constructor. The actual business logic
    /// of "registering" a customer (validation, persistence, notifications)
    /// would be in a CustomerService, following SRP.
    pub fn new(name: String, email: String, phone: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            phone,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_customer() {
        let customer = Customer::new(
            "Alice".to_string(),
            "alice@example.com".to_string(),
            Some("+1234567890".to_string()),
        );

        assert_eq!(customer.name, "Alice");
        assert_eq!(customer.email, "alice@example.com");
    }
}
