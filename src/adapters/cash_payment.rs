// SOLID: CashPayment - Cash payment processor adapter
// 
// PRINCIPLES DEMONSTRATED:
// 
// 1. OPEN-CLOSED PRINCIPLE (OCP):
//    This is one payment method. We can add others (CreditCard, Mobile, Crypto)
//    without modifying this code or OrderService
// 
// 2. LISKOV SUBSTITUTION PRINCIPLE (LSP):
//    This must be substitutable for any other PaymentProcessor
//    OrderService shouldn't care if it's Cash, CreditCard, or Bitcoin
// 
// 3. DEPENDENCY INVERSION PRINCIPLE (DIP):
//    This adapter depends on the PaymentProcessor trait

use crate::ports::{PaymentError, PaymentProcessor};
use uuid::Uuid;

/// Cash payment processor
/// 
/// SOLID PRINCIPLE: Open-Closed Principle (OCP)
/// 
/// This is ONE payment method. To add more, we just create new types:
/// - CreditCardPayment
/// - MobilePayment
/// - BitcoinPayment
/// - PayPalPayment
/// 
/// Each is independent. Adding one doesn't require changing others.
/// OrderService doesn't need to change - it works with the PaymentProcessor trait.
/// 
/// SOLID PRINCIPLE: Liskov Substitution Principle (LSP)
/// 
/// Any code using PaymentProcessor should work with Cash or any other implementation.
/// The contract is:
/// - Input: amount to charge
/// - Output: Ok(payment_id) or Err(PaymentError)
/// - No side effects on orders or storage (that's OrderService's job)
pub struct CashPayment;

impl PaymentProcessor for CashPayment {
    fn process_payment(&self, amount: f64) -> Result<String, PaymentError> {
        // Simulate cash payment processing
        println!("💵 Processing cash payment of ${:.2}", amount);

        // In a real system, this might:
        // - Record in a payment ledger
        // - Generate a receipt number
        // - Update cash register balance
        // For this demo, we just generate a payment ID

        // LSP: Return Ok with a unique payment ID (the contract)
        let payment_id = format!("CASH-{}", Uuid::new_v4());
        println!("✓ Cash payment successful: {}", payment_id);

        Ok(payment_id)
    }

    fn payment_method_name(&self) -> &str {
        "Cash"
    }
}

// ============================================================================
// OCP IN ACTION: Adding New Payment Methods
// 
// Suppose we want to add credit card payment. Here's ALL the code needed:
// 
// pub struct CreditCardPayment {
//     card_processor: CardProcessor, // External payment gateway
// }
// 
// impl PaymentProcessor for CreditCardPayment {
//     fn process_payment(&self, amount: f64) -> Result<String, PaymentError> {
//         println!("💳 Processing credit card payment of ${:.2}", amount);
//         
//         // Call external payment gateway
//         let result = self.card_processor.charge(amount)?;
//         
//         Ok(result.transaction_id)
//     }
//     
//     fn payment_method_name(&self) -> &str {
//         "Credit Card"
//     }
// }
// 
// Then in main.rs:
// let payment = CreditCardPayment::new(card_processor);
// let service = OrderService::new(repo, payment, notifier);
// 
// OrderService doesn't change. Cash payment doesn't change. OCP achieved!
// ============================================================================

// ============================================================================
// LSP IN ACTION: Substitutability
// 
// This code should work with ANY PaymentProcessor:
// 
// fn charge_customer<P: PaymentProcessor>(
//     processor: &P,
//     amount: f64,
// ) -> Result<String, PaymentError> {
//     processor.process_payment(amount)
// }
// 
// charge_customer(&CashPayment, 10.00);
// charge_customer(&CreditCardPayment::new(...), 10.00);
// charge_customer(&BitcoinPayment::new(...), 10.00);
// 
// All should work identically from the caller's perspective.
// Different implementations, same interface, consistent behavior. That's LSP.
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cash_payment_success() {
        let payment = CashPayment;
        let result = payment.process_payment(10.50);

        assert!(result.is_ok());
        let payment_id = result.unwrap();
        assert!(payment_id.starts_with("CASH-"));
    }

    #[test]
    fn test_payment_method_name() {
        let payment = CashPayment;
        assert_eq!(payment.payment_method_name(), "Cash");
    }
}
