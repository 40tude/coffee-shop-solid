// SOLID: ConsoleNotifier - Console notification adapter
// 
// PRINCIPLES DEMONSTRATED:
// 
// 1. INTERFACE SEGREGATION PRINCIPLE (ISP):
//    This adapter only implements Notifier (focused interface)
//    It doesn't need to implement payment, storage, or other concerns
// 
// 2. DEPENDENCY INVERSION PRINCIPLE (DIP):
//    This adapter depends on the Notifier trait (abstraction)
// 
// 3. SINGLE RESPONSIBILITY PRINCIPLE (SRP):
//    This has ONE job: send notifications to the console
//    It doesn't handle order creation, payment, or storage

use crate::domain::Order;
use crate::ports::{NotificationError, Notifier};

/// Console notifier - prints notifications to stdout
/// 
/// SOLID PRINCIPLE: Interface Segregation Principle (ISP)
/// 
/// This type only implements Notifier. It doesn't implement:
/// - OrderRepository (not its job)
/// - PaymentProcessor (not its job)
/// - PricingCalculator (not its job)
/// 
/// ISP says: "Don't depend on methods you don't use."
/// 
/// If we had one giant interface with all these methods,
/// ConsoleNotifier would be forced to implement things it doesn't need.
/// 
/// With ISP, we have small, focused interfaces. Each type implements
/// only what it needs.
/// 
/// SOLID PRINCIPLE: Single Responsibility Principle (SRP)
/// 
/// This struct has ONE responsibility: send console notifications.
/// 
/// If we want to add email notifications, we create EmailNotifier.
/// If we want SMS, we create SmsNotifier.
/// 
/// Each is independent and has its own single responsibility.
/// 
/// USE CASE:
/// Perfect for:
/// - Development (immediate feedback)
/// - Demos (visual confirmation)
/// - Testing (easy to verify output)
pub struct ConsoleNotifier;

impl Notifier for ConsoleNotifier {
    fn notify_order_placed(&self, order: &Order) -> Result<(), NotificationError> {
        // Format notification message
        let message = format!(
            "🎉 Order Placed!\n\
             Order ID: {}\n\
             Customer: {} ({})\n\
             Items: {}\n\
             Total: ${:.2}\n\
             Status: {:?}",
            order.id,
            order.customer.name,
            order.customer.email,
            order.items.len(),
            order.total_price,
            order.status
        );

        // Print to console
        println!("\n{}\n", message);

        Ok(())
    }

    fn notify_order_ready(&self, order: &Order) -> Result<(), NotificationError> {
        let message = format!(
            "☕ Order Ready for Pickup!\n\
             Order ID: {}\n\
             Customer: {}\n\
             Please come to the counter!",
            order.id, order.customer.name
        );

        println!("\n{}\n", message);

        Ok(())
    }

    fn notify_order_cancelled(&self, order: &Order) -> Result<(), NotificationError> {
        let message = format!(
            "❌ Order Cancelled\n\
             Order ID: {}\n\
             Customer: {}",
            order.id, order.customer.name
        );

        println!("\n{}\n", message);

        Ok(())
    }
}

// ============================================================================
// ISP IN ACTION: Focused Interfaces
// 
// Compare these two approaches:
// 
// BAD (Fat Interface):
// trait OrderManager {
//     fn save_order(&mut self, order: &Order) -> Result<()>;
//     fn process_payment(&self, amount: f64) -> Result<String>;
//     fn notify_customer(&self, order: &Order) -> Result<()>;
//     fn calculate_price(&self, items: &[Item]) -> f64;
//     fn generate_report(&self) -> Report;
// }
// 
// impl OrderManager for ConsoleNotifier {
//     fn save_order(&mut self, order: &Order) -> Result<()> {
//         unimplemented!() // Not needed!
//     }
//     fn process_payment(&self, amount: f64) -> Result<String> {
//         unimplemented!() // Not needed!
//     }
//     fn notify_customer(&self, order: &Order) -> Result<()> {
//         // Only this is needed
//     }
//     fn calculate_price(&self, items: &[Item]) -> f64 {
//         unimplemented!() // Not needed!
//     }
//     fn generate_report(&self) -> Report {
//         unimplemented!() // Not needed!
//     }
// }
// 
// GOOD (ISP - Focused Interfaces):
// trait Notifier {
//     fn notify_customer(&self, order: &Order) -> Result<()>;
// }
// 
// impl Notifier for ConsoleNotifier {
//     fn notify_customer(&self, order: &Order) -> Result<()> {
//         // Just implement what we need!
//     }
// }
// 
// With ISP, ConsoleNotifier only implements what it needs.
// No unnecessary methods. No unimplemented!() calls. Clean and focused.
// ============================================================================

// ============================================================================
// OCP + ISP: Adding New Notification Channels
// 
// Want to add email notifications? Here's the complete code:
// 
// pub struct EmailNotifier {
//     smtp_server: String,
//     from_address: String,
// }
// 
// impl Notifier for EmailNotifier {
//     fn notify_order_placed(&self, order: &Order) -> Result<(), NotificationError> {
//         let email = format!(
//             "To: {}\n\
//              Subject: Order Confirmation\n\
//              \n\
//              Your order {} has been placed!",
//             order.customer.email,
//             order.id
//         );
//         
//         // Send via SMTP
//         self.send_email(&email)?;
//         Ok(())
//     }
//     // ... other methods
// }
// 
// Then in main.rs:
// let notifier = EmailNotifier::new(...);
// let service = OrderService::new(repo, payment, notifier);
// 
// No changes to OrderService. No changes to ConsoleNotifier.
// The system was open for this extension. OCP + ISP working together!
// ============================================================================

// ============================================================================
// COMPOSITE PATTERN: Multiple Notification Channels
// 
// Want to send notifications to multiple channels? Use a composite:
// 
// pub struct CompositeNotifier {
//     notifiers: Vec<Box<dyn Notifier>>,
// }
// 
// impl Notifier for CompositeNotifier {
//     fn notify_order_placed(&self, order: &Order) -> Result<(), NotificationError> {
//         for notifier in &self.notifiers {
//             // Try all notifiers, ignore individual failures
//             let _ = notifier.notify_order_placed(order);
//         }
//         Ok(())
//     }
//     // ... other methods
// }
// 
// let notifiers = vec![
//     Box::new(ConsoleNotifier) as Box<dyn Notifier>,
//     Box::new(EmailNotifier::new(...)),
//     Box::new(SmsNotifier::new(...)),
// ];
// 
// let composite = CompositeNotifier { notifiers };
// let service = OrderService::new(repo, payment, composite);
// 
// Now orders trigger console, email, AND SMS notifications!
// No changes to OrderService needed. That's the power of SOLID.
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Customer, OrderItem};

    fn make_test_order() -> Order {
        let customer = Customer::new(
            "Test User".to_string(),
            "test@example.com".to_string(),
            Some("+1234567890".to_string()),
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
    fn test_notify_order_placed() {
        let notifier = ConsoleNotifier;
        let order = make_test_order();

        let result = notifier.notify_order_placed(&order);
        assert!(result.is_ok());
    }

    #[test]
    fn test_notify_order_ready() {
        let notifier = ConsoleNotifier;
        let mut order = make_test_order();
        order.mark_as_paid("TEST-123".to_string());
        order.mark_as_preparing();
        order.mark_as_ready();

        let result = notifier.notify_order_ready(&order);
        assert!(result.is_ok());
    }

    #[test]
    fn test_notify_order_cancelled() {
        let notifier = ConsoleNotifier;
        let mut order = make_test_order();
        order.cancel();

        let result = notifier.notify_order_cancelled(&order);
        assert!(result.is_ok());
    }
}
