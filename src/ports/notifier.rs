// SOLID: This module defines the Notifier PORT (abstraction)
// 
// PRINCIPLES DEMONSTRATED:
// 
// 1. INTERFACE SEGREGATION PRINCIPLE (ISP):
//    This trait is FOCUSED on sending notifications ONLY
//    It doesn't handle orders, payments, or storage
//    Components that need notifications depend only on this interface
// 
// 2. DEPENDENCY INVERSION PRINCIPLE (DIP):
//    OrderService depends on this trait, not on Console, Email, or SMS implementations
// 
// 3. SINGLE RESPONSIBILITY PRINCIPLE (SRP):
//    Notification is a separate concern from order management, payment, etc.
//    This trait isolates that concern

use crate::domain::Order;
use std::error::Error;
use std::fmt;

/// Error type for notification operations
#[derive(Debug, Clone)]
pub enum NotificationError {
    SendFailed(String),
    InvalidRecipient(String),
    NetworkError(String),
}

impl fmt::Display for NotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotificationError::SendFailed(msg) => write!(f, "Send failed: {}", msg),
            NotificationError::InvalidRecipient(msg) => write!(f, "Invalid recipient: {}", msg),
            NotificationError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl Error for NotificationError {}

/// Notifier trait for sending notifications
/// 
/// SOLID PRINCIPLES:
/// 
/// 1. INTERFACE SEGREGATION PRINCIPLE (ISP):
///    This is a SMALL, FOCUSED interface. It only handles notifications.
///    
///    Compare this to a "god interface":
///    trait OrderManager {
///        fn save_order(...);
///        fn process_payment(...);
///        fn send_notification(...);
///        fn generate_report(...);
///        fn calculate_tax(...);
///    }
///    
///    The problem with god interfaces:
///    - A simple console notifier would have to implement ALL methods
///    - Changes to payment logic affect notification implementations
///    - Testing requires mocking everything
///    
///    With ISP:
///    - ConsoleNotifier only implements what it needs
///    - EmailNotifier is independent of SMS notifier
///    - Each interface is cohesive and focused
/// 
/// 2. SINGLE RESPONSIBILITY PRINCIPLE (SRP):
///    Notification is ONE responsibility, separated from:
///    - Order management (OrderService's job)
///    - Payment processing (PaymentProcessor's job)
///    - Storage (OrderRepository's job)
///    
///    This separation means:
///    - If notification requirements change, only Notifier implementations change
///    - If we switch from email to SMS, OrderService doesn't change
///    - Different teams can own different concerns
/// 
/// 3. DEPENDENCY INVERSION PRINCIPLE (DIP):
///    High-level code (OrderService) depends on this abstraction.
///    Low-level implementations (Console, Email, SMS) also depend on this abstraction.
///    
///    This allows:
///    - Testing with mock notifiers
///    - Swapping notification channels without touching business logic
///    - Running the app without email server (use Console instead)
pub trait Notifier {
    /// Notify customer that their order was placed successfully
    /// 
    /// CONTRACT:
    /// - Should be non-blocking (don't slow down order processing)
    /// - If notification fails, log it but don't fail the order
    /// - Returns Ok(()) if sent, Err if failed
    fn notify_order_placed(&self, order: &Order) -> Result<(), NotificationError>;

    /// Notify customer that their order is ready for pickup
    fn notify_order_ready(&self, order: &Order) -> Result<(), NotificationError>;

    /// Notify customer that their order was cancelled
    fn notify_order_cancelled(&self, order: &Order) -> Result<(), NotificationError>;
}

// ============================================================================
// ISP IN ACTION: Composition Over Fat Interfaces
// 
// Instead of one giant interface with 20 methods, we have small, focused traits.
// 
// If we need multiple capabilities, we can compose them:
// 
// fn process_order<N>(notifier: &N, order: &Order)
// where
//     N: Notifier + Logger + MetricsCollector
// {
//     notifier.notify_order_placed(order)?;
//     notifier.log_event("order_placed", order.id)?;
//     notifier.record_metric("orders", 1)?;
// }
// 
// Each trait is focused. Types implement only what they need.
// 
// Benefits:
// - A simple ConsoleNotifier only implements Notifier
// - A complex CloudNotifier might implement all three
// - No component is forced to implement irrelevant methods
// ============================================================================

// ============================================================================
// REAL-WORLD EXAMPLE: Notification Channels
// 
// In a real coffee shop app, we might have:
// 
// 1. ConsoleNotifier (for development/testing)
// 2. EmailNotifier (sends via SMTP)
// 3. SmsNotifier (sends via Twilio)
// 4. PushNotifier (sends mobile push notifications)
// 5. CompositeNotifier (sends to multiple channels)
// 
// With ISP, each is independent:
// 
// pub struct CompositeNotifier {
//     notifiers: Vec<Box<dyn Notifier>>,
// }
// 
// impl Notifier for CompositeNotifier {
//     fn notify_order_placed(&self, order: &Order) -> Result<(), NotificationError> {
//         for notifier in &self.notifiers {
//             // Ignore individual failures, try all channels
//             let _ = notifier.notify_order_placed(order);
//         }
//         Ok(())
//     }
//     // ... other methods
// }
// 
// Now we can send notifications to multiple channels simultaneously,
// without any changes to OrderService!
// ============================================================================
