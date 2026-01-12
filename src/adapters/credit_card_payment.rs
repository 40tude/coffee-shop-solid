// SOLID: CreditCardPayment - Credit card payment processor adapter
// 
// OPEN-CLOSED PRINCIPLE (OCP):
// This is a NEW payment method added WITHOUT modifying:
// - PaymentProcessor trait (it was closed)
// - OrderService (it depends on the trait)
// - CashPayment (it's independent)
// 
// This demonstrates how OCP enables extension without modification.

use crate::ports::{PaymentError, PaymentProcessor};
use uuid::Uuid;

/// Credit card payment processor
/// 
/// SOLID PRINCIPLE: Open-Closed Principle (OCP)
/// 
/// We added this payment method WITHOUT modifying:
/// - The PaymentProcessor trait (closed)
/// - OrderService (works with any PaymentProcessor)
/// - CashPayment (independent)
/// 
/// This is the power of OCP: extend behavior by adding code, not changing it.
/// 
/// SOLID PRINCIPLE: Liskov Substitution Principle (LSP)
/// 
/// This must be substitutable for CashPayment or any other PaymentProcessor.
/// From OrderService's perspective, it shouldn't matter which payment method is used.
/// The interface and behavior must be consistent.
pub struct CreditCardPayment {
    // In a real system, this would hold:
    // - Connection to payment gateway (Stripe, Square, etc.)
    // - API credentials
    // - Configuration
    _gateway_url: String,
}

impl CreditCardPayment {
    /// Create a new credit card payment processor
    pub fn new(gateway_url: String) -> Self {
        Self {
            _gateway_url: gateway_url,
        }
    }
}

impl PaymentProcessor for CreditCardPayment {
    fn process_payment(&self, amount: f64) -> Result<String, PaymentError> {
        // Simulate credit card payment processing
        println!("💳 Processing credit card payment of ${:.2}", amount);

        // In a real system, this would:
        // 1. Validate card details
        // 2. Call payment gateway API
        // 3. Handle 3D Secure if needed
        // 4. Record transaction
        // 5. Return transaction ID

        // Simulate validation
        if amount < 0.0 {
            return Err(PaymentError::ProcessingFailed(
                "Amount cannot be negative".to_string(),
            ));
        }

        // Simulate occasional failures (for demo purposes)
        // In real code, failures would come from the payment gateway
        if amount > 1000.0 {
            return Err(PaymentError::ProcessingFailed(
                "Amount exceeds card limit".to_string(),
            ));
        }

        // LSP: Return Ok with a unique payment ID (honoring the contract)
        let payment_id = format!("CC-{}", Uuid::new_v4());
        println!("✓ Credit card payment successful: {}", payment_id);

        Ok(payment_id)
    }

    fn payment_method_name(&self) -> &str {
        "Credit Card"
    }
}

// ============================================================================
// OCP EXAMPLE: Adding Yet Another Payment Method
// 
// Want to add Mobile Payment (Apple Pay, Google Pay)?
// Here's the complete code needed:
// 
// pub struct MobilePayment {
//     provider: String, // "Apple Pay", "Google Pay", etc.
// }
// 
// impl PaymentProcessor for MobilePayment {
//     fn process_payment(&self, amount: f64) -> Result<String, PaymentError> {
//         println!("📱 Processing {} payment of ${:.2}", self.provider, amount);
//         Ok(format!("MOBILE-{}", Uuid::new_v4()))
//     }
//     
//     fn payment_method_name(&self) -> &str {
//         &self.provider
//     }
// }
// 
// That's it! No changes to:
// - OrderService
// - CashPayment
// - CreditCardPayment
// - Any other code
// 
// The system is OPEN for this extension but was CLOSED to modification.
// ============================================================================

// ============================================================================
// REAL-WORLD BENEFIT: Payment Method Strategies
// 
// In a real coffee shop app, you might want to choose payment methods dynamically:
// 
// enum PaymentMethod {
//     Cash,
//     CreditCard,
//     Mobile(String),
//     Cryptocurrency(String),
// }
// 
// impl PaymentMethod {
//     fn to_processor(&self) -> Box<dyn PaymentProcessor> {
//         match self {
//             PaymentMethod::Cash => Box::new(CashPayment),
//             PaymentMethod::CreditCard => Box::new(CreditCardPayment::new(...)),
//             PaymentMethod::Mobile(p) => Box::new(MobilePayment::new(p.clone())),
//             PaymentMethod::Cryptocurrency(c) => Box::new(CryptoPayment::new(c.clone())),
//         }
//     }
// }
// 
// let processor = user_selected_method.to_processor();
// let service = OrderService::new(repo, processor, notifier);
// 
// Each payment method is independently implemented and testable.
// Adding a new one doesn't affect existing ones. That's OCP in production!
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_card_payment_success() {
        let payment = CreditCardPayment::new("https://payment-gateway.example.com".to_string());
        let result = payment.process_payment(50.00);

        assert!(result.is_ok());
        let payment_id = result.unwrap();
        assert!(payment_id.starts_with("CC-"));
    }

    #[test]
    fn test_credit_card_payment_negative_amount() {
        let payment = CreditCardPayment::new("https://payment-gateway.example.com".to_string());
        let result = payment.process_payment(-10.00);

        assert!(result.is_err());
    }

    #[test]
    fn test_credit_card_payment_exceeds_limit() {
        let payment = CreditCardPayment::new("https://payment-gateway.example.com".to_string());
        let result = payment.process_payment(1500.00);

        assert!(result.is_err());
    }

    #[test]
    fn test_payment_method_name() {
        let payment = CreditCardPayment::new("https://payment-gateway.example.com".to_string());
        assert_eq!(payment.payment_method_name(), "Credit Card");
    }
}
