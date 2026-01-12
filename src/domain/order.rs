// SOLID: Order is a domain entity (pure business concept)
// It has NO dependencies on infrastructure
// It doesn't know how to save itself, send notifications, or process payments
// Those are responsibilities of other modules (SRP)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::customer::Customer;

/// Status of an order in its lifecycle
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,    // Just created
    Paid,       // Payment successful
    Preparing,  // Barista is making it
    Ready,      // Ready for pickup
    Completed,  // Customer picked it up
    Cancelled,  // Order was cancelled
}

/// Represents an order in our coffee shop
/// 
/// SOLID PRINCIPLE: Single Responsibility Principle (SRP)
/// 
/// This struct has ONE responsibility: represent an order's data and state.
/// It does NOT:
/// - Save itself to storage (Repository's job)
/// - Calculate its own price (PricingCalculator's job)
/// - Process payments (PaymentProcessor's job)
/// - Send notifications (Notifier's job)
/// - Format itself for display (Formatter's job)
/// 
/// Each of those is a SEPARATE responsibility handled by a SEPARATE module.
/// This makes the code easier to maintain and test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub customer: Customer,
    pub items: Vec<OrderItem>,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub total_price: f64,
    pub payment_id: Option<String>,
}

/// An item in an order
/// 
/// Note: We store the beverage name and price at the time of order
/// (not a reference to a Beverage trait object, which wouldn't be serializable)
/// This is a pragmatic choice - in a real system, you might want to store
/// the beverage details differently.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub beverage_name: String,
    pub beverage_description: String,
    pub price: f64,
    pub quantity: u8,
}

impl Order {
    /// Create a new order
    /// 
    /// SOLID: This is just a constructor. The actual business logic of
    /// "placing an order" (validation, payment, persistence, notification)
    /// is in OrderService, following SRP.
    pub fn new(customer: Customer, items: Vec<OrderItem>) -> Self {
        let total_price = items.iter()
            .map(|item| item.price * item.quantity as f64)
            .sum();

        Self {
            id: Uuid::new_v4(),
            customer,
            items,
            status: OrderStatus::Pending,
            created_at: Utc::now(),
            total_price,
            payment_id: None,
        }
    }

    /// Mark order as paid
    /// 
    /// SOLID: Notice this is just a state transition method.
    /// The actual payment processing logic is elsewhere (SRP).
    pub fn mark_as_paid(&mut self, payment_id: String) {
        self.status = OrderStatus::Paid;
        self.payment_id = Some(payment_id);
    }

    /// Mark order as preparing
    pub fn mark_as_preparing(&mut self) {
        if self.status == OrderStatus::Paid {
            self.status = OrderStatus::Preparing;
        }
    }

    /// Mark order as ready
    pub fn mark_as_ready(&mut self) {
        if self.status == OrderStatus::Preparing {
            self.status = OrderStatus::Ready;
        }
    }

    /// Mark order as completed
    pub fn mark_as_completed(&mut self) {
        if self.status == OrderStatus::Ready {
            self.status = OrderStatus::Completed;
        }
    }

    /// Cancel order
    pub fn cancel(&mut self) {
        if self.status != OrderStatus::Completed {
            self.status = OrderStatus::Cancelled;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::customer::Customer;

    fn make_test_customer() -> Customer {
        Customer::new(
            "Test User".to_string(),
            "test@example.com".to_string(),
            None,
        )
    }

    fn make_test_item() -> OrderItem {
        OrderItem {
            beverage_name: "Coffee".to_string(),
            beverage_description: "Medium Coffee".to_string(),
            price: 3.50,
            quantity: 1,
        }
    }

    #[test]
    fn test_create_order() {
        let customer = make_test_customer();
        let items = vec![make_test_item()];
        
        let order = Order::new(customer, items);
        
        assert_eq!(order.status, OrderStatus::Pending);
        assert_eq!(order.total_price, 3.50);
    }

    #[test]
    fn test_order_workflow() {
        let customer = make_test_customer();
        let items = vec![make_test_item()];
        let mut order = Order::new(customer, items);

        // Start as pending
        assert_eq!(order.status, OrderStatus::Pending);

        // Mark as paid
        order.mark_as_paid("PAY-123".to_string());
        assert_eq!(order.status, OrderStatus::Paid);
        assert_eq!(order.payment_id, Some("PAY-123".to_string()));

        // Preparing
        order.mark_as_preparing();
        assert_eq!(order.status, OrderStatus::Preparing);

        // Ready
        order.mark_as_ready();
        assert_eq!(order.status, OrderStatus::Ready);

        // Completed
        order.mark_as_completed();
        assert_eq!(order.status, OrderStatus::Completed);
    }

    #[test]
    fn test_total_price_calculation() {
        let customer = make_test_customer();
        let items = vec![
            OrderItem {
                beverage_name: "Coffee".to_string(),
                beverage_description: "Medium Coffee".to_string(),
                price: 3.50,
                quantity: 2,
            },
            OrderItem {
                beverage_name: "Tea".to_string(),
                beverage_description: "Large Green Tea".to_string(),
                price: 3.00,
                quantity: 1,
            },
        ];
        
        let order = Order::new(customer, items);
        
        // (3.50 * 2) + (3.00 * 1) = 10.00
        assert_eq!(order.total_price, 10.00);
    }
}
