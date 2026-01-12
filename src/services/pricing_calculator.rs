// SOLID: This module demonstrates the Single Responsibility Principle (SRP)
// 
// PricingCalculator has ONE responsibility: calculate prices
// It does NOT:
// - Store orders (Repository's job)
// - Process payments (PaymentProcessor's job)
// - Send notifications (Notifier's job)
// - Manage order workflow (OrderService's job)
// 
// SRP BENEFIT:
// If pricing rules change (e.g., new tax rate, discount system, happy hour pricing),
// ONLY this module changes. Other modules are unaffected.
// 
// ACTOR: The Accounting department owns this module.
// If Accounting wants to change pricing rules, they change this file and nothing else.

use crate::domain::Beverage;

/// Pricing calculator
/// 
/// SOLID PRINCIPLE: Single Responsibility Principle (SRP)
/// 
/// This struct has ONE reason to change: when pricing rules change.
/// 
/// In Uncle Bob's terms:
/// "A module should be responsible to one, and only one, actor."
/// 
/// The ACTOR here is the Accounting department. They own pricing rules.
/// If Operations wants to change order workflow, they don't touch this.
/// If IT wants to change storage, they don't touch this.
/// If Marketing wants to change notifications, they don't touch this.
/// 
/// This separation prevents:
/// - Merge conflicts between teams working on different concerns
/// - Accidental coupling (changing pricing shouldn't break notifications)
/// - Ripple effects (a pricing bug shouldn't affect order storage)
pub struct PricingCalculator {
    tax_rate: f64,
}

impl PricingCalculator {
    /// Create a new pricing calculator with a tax rate
    /// 
    /// Example: 0.08 for 8% tax
    pub fn new(tax_rate: f64) -> Self {
        Self { tax_rate }
    }

    /// Calculate price for a beverage
    /// 
    /// This method encapsulates the pricing logic.
    /// If we need to add complexity (discounts, loyalty points, happy hour),
    /// we change THIS METHOD, not the callers.
    pub fn calculate_beverage_price(&self, beverage: &dyn Beverage) -> f64 {
        beverage.price()
    }

    /// Calculate price with tax
    pub fn calculate_price_with_tax(&self, base_price: f64) -> f64 {
        base_price * (1.0 + self.tax_rate)
    }

    /// Calculate total for an order
    /// 
    /// This demonstrates SRP in action:
    /// - This calculator doesn't know about Order struct
    /// - It takes a list of beverages (generic)
    /// - It has no side effects (pure calculation)
    /// - It doesn't save, notify, or process payments
    pub fn calculate_total(&self, beverages: &[&dyn Beverage]) -> f64 {
        let subtotal: f64 = beverages
            .iter()
            .map(|b| self.calculate_beverage_price(*b))
            .sum();

        self.calculate_price_with_tax(subtotal)
    }

    /// Apply a discount
    /// 
    /// Example extension: if we want to add discount logic later,
    /// we add it here. No other code needs to change.
    pub fn apply_discount(&self, price: f64, discount_percent: f64) -> f64 {
        price * (1.0 - discount_percent / 100.0)
    }

    /// Calculate loyalty discount
    /// 
    /// Example: every 10th order gets 10% off
    /// This is a separate method because it's a separate pricing rule.
    /// If this rule changes, we change this method only.
    pub fn calculate_loyalty_discount(&self, order_count: u32) -> f64 {
        if order_count > 0 && order_count % 10 == 0 {
            10.0 // 10% off
        } else {
            0.0
        }
    }
}

// ============================================================================
// WHY SRP MATTERS: A Counter-Example
// 
// Imagine if we DIDN'T follow SRP and put everything in one place:
// 
// struct OrderManager {
//     db: Database,
//     payment_gateway: PaymentGateway,
//     email_service: EmailService,
//     tax_rate: f64,
// }
// 
// impl OrderManager {
//     fn process_order(&mut self, order: Order) -> Result<()> {
//         // Calculate price (Accounting's concern)
//         let price = self.calculate_price(&order);
//         
//         // Process payment (Finance's concern)
//         self.payment_gateway.charge(price)?;
//         
//         // Save to DB (DBA's concern)
//         self.db.save(&order)?;
//         
//         // Send email (Marketing's concern)
//         self.email_service.send(&order)?;
//         
//         Ok(())
//     }
// }
// 
// PROBLEMS:
// 1. Four different teams need to modify this file for unrelated changes
// 2. Merge conflicts galore
// 3. Changes by Accounting (pricing) might break email sending
// 4. Testing requires mocking database, payment gateway, and email service
// 5. Can't reuse pricing logic elsewhere
// 
// WITH SRP:
// - PricingCalculator (this file) - Accounting's responsibility
// - PaymentProcessor - Finance's responsibility
// - OrderRepository - DBA's responsibility
// - Notifier - Marketing's responsibility
// - OrderService - Operations' responsibility (coordinates them)
// 
// Each team owns their module. No conflicts. Easy to test. Easy to reuse.
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Coffee, Size};

    #[test]
    fn test_calculate_price_with_tax() {
        let calculator = PricingCalculator::new(0.08); // 8% tax
        let price_with_tax = calculator.calculate_price_with_tax(100.0);
        
        assert_eq!(price_with_tax, 108.0);
    }

    #[test]
    fn test_beverage_pricing() {
        let calculator = PricingCalculator::new(0.08);
        let coffee = Coffee {
            size: Size::Medium,
            extra_shots: 0,
        };

        let price = calculator.calculate_beverage_price(&coffee);
        
        // Coffee base: 3.50, Medium: 1.0 multiplier
        assert_eq!(price, 3.50);
    }

    #[test]
    fn test_discount() {
        let calculator = PricingCalculator::new(0.08);
        let discounted = calculator.apply_discount(100.0, 10.0);
        
        // 100 - 10% = 90
        assert_eq!(discounted, 90.0);
    }

    #[test]
    fn test_loyalty_discount() {
        let calculator = PricingCalculator::new(0.08);
        
        assert_eq!(calculator.calculate_loyalty_discount(9), 0.0);
        assert_eq!(calculator.calculate_loyalty_discount(10), 10.0);
        assert_eq!(calculator.calculate_loyalty_discount(11), 0.0);
        assert_eq!(calculator.calculate_loyalty_discount(20), 10.0);
    }

    #[test]
    fn test_calculate_total() {
        let calculator = PricingCalculator::new(0.10); // 10% tax
        
        let coffee1 = Coffee { size: Size::Small, extra_shots: 0 };
        let coffee2 = Coffee { size: Size::Medium, extra_shots: 1 };
        
        let beverages: Vec<&dyn Beverage> = vec![&coffee1, &coffee2];
        let total = calculator.calculate_total(&beverages);
        
        // Coffee1: 3.50 * 0.8 = 2.80
        // Coffee2: 4.25 * 1.0 = 4.25
        // Subtotal: 7.05
        // With 10% tax: 7.755
        assert!((total - 7.755).abs() < 0.01);
    }
}
