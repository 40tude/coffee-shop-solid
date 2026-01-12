// SOLID: OrderService - The Application Service Layer
// 
// This is where SOLID principles come together beautifully.
// 
// PRINCIPLES DEMONSTRATED:
// 
// 1. DEPENDENCY INVERSION PRINCIPLE (DIP):
//    This service depends on ABSTRACTIONS (traits), not concrete implementations
//    - OrderRepository trait (not MemoryStorage or JsonStorage)
//    - PaymentProcessor trait (not CashPayment or CreditCard)
//    - Notifier trait (not ConsoleNotifier or EmailNotifier)
// 
// 2. SINGLE RESPONSIBILITY PRINCIPLE (SRP):
//    This service has ONE responsibility: orchestrate the order workflow
//    It doesn't calculate prices, process payments, or send notifications itself
//    It COORDINATES those responsibilities
// 
// 3. OPEN-CLOSED PRINCIPLE (OCP):
//    We can add new payment methods, storage backends, or notification channels
//    without modifying this service
// 
// 4. INTERFACE SEGREGATION PRINCIPLE (ISP):
//    This service depends on three small, focused interfaces
//    Not on one giant "OrderManager" interface with 20 methods

use crate::domain::{Beverage, Customer, Order, OrderItem, OrderStatus};
use crate::ports::{
    Notifier, NotificationError, OrderRepository, PaymentError, PaymentProcessor, RepositoryError,
};
use std::error::Error;
use std::fmt;

/// Errors that can occur during order processing
#[derive(Debug)]
pub enum OrderServiceError {
    PaymentFailed(PaymentError),
    StorageFailed(RepositoryError),
    NotificationFailed(NotificationError),
    OrderNotFound,
    InvalidOrder(String),
}

impl fmt::Display for OrderServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderServiceError::PaymentFailed(e) => write!(f, "Payment failed: {}", e),
            OrderServiceError::StorageFailed(e) => write!(f, "Storage failed: {}", e),
            OrderServiceError::NotificationFailed(e) => write!(f, "Notification failed: {}", e),
            OrderServiceError::OrderNotFound => write!(f, "Order not found"),
            OrderServiceError::InvalidOrder(msg) => write!(f, "Invalid order: {}", msg),
        }
    }
}

impl Error for OrderServiceError {}

/// OrderService - Orchestrates the order workflow
/// 
/// SOLID PRINCIPLE: Dependency Inversion Principle (DIP)
/// 
/// This service is GENERIC over:
/// - R: OrderRepository - any storage implementation
/// - P: PaymentProcessor - any payment implementation
/// - N: Notifier - any notification implementation
/// 
/// This means:
/// - We can swap implementations without changing this code
/// - We can test with mock implementations (no database, no payment gateway)
/// - The business logic is INDEPENDENT of infrastructure
/// 
/// SOLID PRINCIPLE: Single Responsibility Principle (SRP)
/// 
/// This service has ONE responsibility: manage the order lifecycle.
/// It coordinates other services but doesn't do their work:
/// - Doesn't calculate prices itself (uses PricingCalculator)
/// - Doesn't store orders itself (uses OrderRepository)
/// - Doesn't process payments itself (uses PaymentProcessor)
/// - Doesn't send notifications itself (uses Notifier)
/// 
/// SOLID PRINCIPLE: Open-Closed Principle (OCP)
/// 
/// Want to add a new payment method? Just pass a new PaymentProcessor.
/// Want to switch from JSON to Postgres? Just pass a new OrderRepository.
/// Want to add SMS notifications? Just pass a new Notifier.
/// 
/// This service requires ZERO changes. It's open for extension, closed for modification.
pub struct OrderService<R, P, N>
where
    R: OrderRepository,
    P: PaymentProcessor,
    N: Notifier,
{
    repository: R,
    payment_processor: P,
    notifier: N,
}

impl<R, P, N> OrderService<R, P, N>
where
    R: OrderRepository,
    P: PaymentProcessor,
    N: Notifier,
{
    /// Create a new OrderService
    /// 
    /// SOLID (DIP): We inject dependencies through the constructor.
    /// This is "Dependency Injection" - the dependencies are provided from outside.
    /// The service doesn't know or care about concrete types.
    pub fn new(repository: R, payment_processor: P, notifier: N) -> Self {
        Self {
            repository,
            payment_processor,
            notifier,
        }
    }

    /// Place a new order
    /// 
    /// This method orchestrates the entire order workflow:
    /// 1. Create the order
    /// 2. Process payment
    /// 3. Save to storage
    /// 4. Send notification
    /// 
    /// SOLID (SRP): Notice this method doesn't DO these things,
    /// it COORDINATES them. Each step is delegated to a specialized component.
    /// 
    /// SOLID (DIP): We depend on traits, so any implementation works.
    /// We're not calling MemoryStorage.save() or CashPayment.charge().
    /// We're calling trait methods, which can be implemented by anything.
    pub fn place_order(
        &mut self,
        customer: Customer,
        beverages: Vec<Box<dyn Beverage>>,
    ) -> Result<Order, OrderServiceError> {
        // Validate order
        if beverages.is_empty() {
            return Err(OrderServiceError::InvalidOrder(
                "Order must contain at least one item".to_string(),
            ));
        }

        // Create order items from beverages
        let items: Vec<OrderItem> = beverages
            .iter()
            .map(|b| OrderItem {
                beverage_name: b.name(),
                beverage_description: b.description(),
                price: b.price(),
                quantity: 1,
            })
            .collect();

        // Create the order
        let mut order = Order::new(customer, items);

        // SOLID (DIP): We're calling a trait method, not a concrete implementation
        // This could be CashPayment, CreditCardPayment, MobilePayment, or MockPayment
        // The service doesn't know or care!
        let payment_id = self
            .payment_processor
            .process_payment(order.total_price)
            .map_err(OrderServiceError::PaymentFailed)?;

        // Mark order as paid
        order.mark_as_paid(payment_id);

        // SOLID (DIP): Again, trait method. Could be Memory, JSON, Postgres, etc.
        self.repository
            .save(&order)
            .map_err(OrderServiceError::StorageFailed)?;

        // SOLID (DIP): Trait method. Could be Console, Email, SMS, Push, etc.
        // Note: We don't fail the order if notification fails - it's already paid and saved
        if let Err(e) = self.notifier.notify_order_placed(&order) {
            eprintln!("Warning: Failed to send notification: {}", e);
        }

        Ok(order)
    }

    /// Get an order by ID
    pub fn get_order(&self, id: uuid::Uuid) -> Result<Order, OrderServiceError> {
        self.repository
            .find_by_id(id)
            .map_err(OrderServiceError::StorageFailed)?
            .ok_or(OrderServiceError::OrderNotFound)
    }

    /// List all orders for a customer
    pub fn list_customer_orders(&self, email: &str) -> Result<Vec<Order>, OrderServiceError> {
        self.repository
            .find_by_customer_email(email)
            .map_err(OrderServiceError::StorageFailed)
    }

    /// Mark order as ready and notify customer
    pub fn mark_order_ready(&mut self, id: uuid::Uuid) -> Result<(), OrderServiceError> {
        let mut order = self.get_order(id)?;

        order.mark_as_ready();

        self.repository
            .update(&order)
            .map_err(OrderServiceError::StorageFailed)?;

        // Send notification (don't fail if notification fails)
        if let Err(e) = self.notifier.notify_order_ready(&order) {
            eprintln!("Warning: Failed to send notification: {}", e);
        }

        Ok(())
    }

    /// Cancel an order
    pub fn cancel_order(&mut self, id: uuid::Uuid) -> Result<(), OrderServiceError> {
        let mut order = self.get_order(id)?;

        // Only allow cancelling if not completed
        if order.status == OrderStatus::Completed {
            return Err(OrderServiceError::InvalidOrder(
                "Cannot cancel completed order".to_string(),
            ));
        }

        order.cancel();

        self.repository
            .update(&order)
            .map_err(OrderServiceError::StorageFailed)?;

        if let Err(e) = self.notifier.notify_order_cancelled(&order) {
            eprintln!("Warning: Failed to send notification: {}", e);
        }

        Ok(())
    }

    /// List all orders
    pub fn list_all_orders(&self) -> Result<Vec<Order>, OrderServiceError> {
        self.repository
            .list_all()
            .map_err(OrderServiceError::StorageFailed)
    }
}

// ============================================================================
// KEY INSIGHT: How DIP Enables Testing
// 
// Because OrderService is generic over traits, we can easily test it:
// 
// #[test]
// fn test_place_order() {
//     let repository = MemoryOrderRepository::new();
//     let payment = MockPaymentProcessor { should_fail: false };
//     let notifier = MockNotifier;
//     
//     let mut service = OrderService::new(repository, payment, notifier);
//     
//     let customer = Customer::new(...);
//     let beverages = vec![Box::new(Coffee { ... })];
//     
//     let order = service.place_order(customer, beverages).unwrap();
//     assert_eq!(order.status, OrderStatus::Paid);
// }
// 
// No database needed. No payment gateway needed. No email server needed.
// Pure business logic testing. That's the power of DIP!
// ============================================================================

// ============================================================================
// KEY INSIGHT: How OCP Enables Extension
// 
// Want to add a new payment method? Here's ALL the code needed:
// 
// pub struct BitcoinPayment;
// 
// impl PaymentProcessor for BitcoinPayment {
//     fn process_payment(&self, amount: f64) -> Result<String, PaymentError> {
//         // Bitcoin payment logic
//         Ok(format!("BTC-{}", Uuid::new_v4()))
//     }
// }
// 
// Then in main.rs:
// let service = OrderService::new(repo, BitcoinPayment, notifier);
// 
// That's it! OrderService doesn't change. Zero modifications. OCP in action.
// ============================================================================

// ============================================================================
// KEY INSIGHT: How SRP Prevents Ripple Effects
// 
// Imagine these scenarios:
// 
// 1. Accounting wants to change pricing rules
//    → Only PricingCalculator changes
//    → OrderService unchanged
// 
// 2. DBA wants to switch from JSON to Postgres
//    → Create PostgresOrderRepository
//    → OrderService unchanged
// 
// 3. Finance wants to add fraud detection to payments
//    → Modify PaymentProcessor implementations
//    → OrderService unchanged
// 
// 4. Marketing wants to add SMS notifications
//    → Create SmsNotifier
//    → OrderService unchanged
// 
// Each concern is isolated. Changes don't ripple through the system.
// That's SRP + DIP working together.
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Coffee, Size};
    use crate::adapters::{MemoryOrderRepository, CashPayment, ConsoleNotifier};

    #[test]
    fn test_place_order_success() {
        let repository = MemoryOrderRepository::new();
        let payment = CashPayment;
        let notifier = ConsoleNotifier;
        let mut service = OrderService::new(repository, payment, notifier);

        let customer = Customer::new(
            "Test User".to_string(),
            "test@example.com".to_string(),
            None,
        );

        let beverages: Vec<Box<dyn Beverage>> = vec![Box::new(Coffee {
            size: Size::Medium,
            extra_shots: 0,
        })];

        let result = service.place_order(customer, beverages);

        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.status, OrderStatus::Paid);
    }

    #[test]
    fn test_place_order_empty_fails() {
        let repository = MemoryOrderRepository::new();
        let payment = CashPayment;
        let notifier = ConsoleNotifier;
        let mut service = OrderService::new(repository, payment, notifier);

        let customer = Customer::new(
            "Test User".to_string(),
            "test@example.com".to_string(),
            None,
        );

        let result = service.place_order(customer, vec![]);

        assert!(result.is_err());
    }
}
