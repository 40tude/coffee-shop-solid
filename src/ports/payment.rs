// SOLID: This module defines the PaymentProcessor PORT (abstraction)
// 
// PRINCIPLES DEMONSTRATED:
// 
// 1. DEPENDENCY INVERSION PRINCIPLE (DIP):
//    High-level business logic depends on this trait, not on concrete payment implementations
// 
// 2. OPEN-CLOSED PRINCIPLE (OCP):
//    To add a new payment method (PayPal, Crypto, etc.), just implement this trait
//    No changes needed to OrderService
// 
// 3. INTERFACE SEGREGATION PRINCIPLE (ISP):
//    This trait is focused ONLY on payment processing
//    It doesn't handle storage, notifications, or other concerns

use std::error::Error;
use std::fmt;

/// Error type for payment operations
/// 
/// SOLID (LSP): All payment processors must use this error type,
/// ensuring they're substitutable and handle errors consistently.
#[derive(Debug, Clone)]
pub enum PaymentError {
    InsufficientFunds,
    InvalidCard,
    ProcessingFailed(String),
    NetworkError(String),
}

impl fmt::Display for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PaymentError::InsufficientFunds => write!(f, "Insufficient funds"),
            PaymentError::InvalidCard => write!(f, "Invalid card"),
            PaymentError::ProcessingFailed(msg) => write!(f, "Processing failed: {}", msg),
            PaymentError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl Error for PaymentError {}

/// Payment processor trait
/// 
/// SOLID PRINCIPLES:
/// 
/// 1. OPEN-CLOSED PRINCIPLE (OCP):
///    Want to add Apple Pay? Google Pay? Cryptocurrency? Just implement this trait.
///    The rest of the system requires ZERO changes.
/// 
/// 2. DEPENDENCY INVERSION PRINCIPLE (DIP):
///    OrderService depends on THIS TRAIT, not on Stripe, PayPal, or any specific processor.
///    This allows us to:
///    - Swap payment providers without touching business logic
///    - Test with a mock payment processor
///    - Support multiple payment methods in the same system
/// 
/// 3. INTERFACE SEGREGATION PRINCIPLE (ISP):
///    This trait is FOCUSED. It only handles payment processing.
///    A payment processor doesn't need to know about:
///    - Storage (not its job)
///    - Notifications (not its job)
///    - Order creation (not its job)
/// 
/// 4. LISKOV SUBSTITUTION PRINCIPLE (LSP):
///    Any implementation should be substitutable.
///    Whether it's Cash, CreditCard, or MobilePayment, the behavior should be consistent:
///    - Same return type (Result<String, PaymentError>)
///    - Same error semantics
///    - Same guarantees (if Ok, payment was successful)
pub trait PaymentProcessor {
    /// Process a payment
    /// 
    /// CONTRACT (important for LSP):
    /// - Takes an amount to charge
    /// - Returns Ok(payment_id) if successful
    /// - Returns Err(PaymentError) if failed
    /// - MUST be idempotent (calling twice with same data should not double-charge)
    /// - MUST NOT modify order state (that's OrderService's job - SRP)
    /// 
    /// The payment_id is a unique identifier for the transaction,
    /// which can be used for refunds, auditing, etc.
    fn process_payment(&self, amount: f64) -> Result<String, PaymentError>;

    /// Get the name of this payment method (for display purposes)
    /// 
    /// This is a default implementation that can be overridden.
    fn payment_method_name(&self) -> &str {
        "Unknown Payment Method"
    }
}

// ============================================================================
// EXAMPLE: How OCP Works Here
// 
// Suppose we start with Cash and CreditCard payment processors.
// Later, we want to add MobilePayment (Apple Pay, Google Pay).
// 
// WITHOUT OCP:
// - Modify OrderService to add a new branch in payment logic
// - Risk breaking existing payment methods
// - Every new payment method requires code changes everywhere
// 
// WITH OCP (this design):
// - Create MobilePayment struct
// - Implement PaymentProcessor trait
// - Pass it to OrderService
// - Done! Zero changes to OrderService or other code
// 
// Code example:
// 
// pub struct MobilePayment {
//     provider: String, // "Apple Pay", "Google Pay", etc.
// }
// 
// impl PaymentProcessor for MobilePayment {
//     fn process_payment(&self, amount: f64) -> Result<String, PaymentError> {
//         // Mobile payment logic here
//         Ok(format!("MOBILE-{}", uuid::Uuid::new_v4()))
//     }
//     
//     fn payment_method_name(&self) -> &str {
//         &self.provider
//     }
// }
// 
// That's it! No changes needed anywhere else. OCP in action.
// ============================================================================

// ============================================================================
// TESTING WITH DIP
// 
// Because OrderService depends on the PaymentProcessor TRAIT (not a concrete type),
// we can easily create a mock for testing:
// 
// pub struct MockPaymentProcessor {
//     pub should_fail: bool,
// }
// 
// impl PaymentProcessor for MockPaymentProcessor {
//     fn process_payment(&self, _amount: f64) -> Result<String, PaymentError> {
//         if self.should_fail {
//             Err(PaymentError::ProcessingFailed("Mock failure".to_string()))
//         } else {
//             Ok("MOCK-12345".to_string())
//         }
//     }
// }
// 
// Now we can test OrderService without any real payment processing!
// ============================================================================
