// SOLID: This module demonstrates the Open-Closed Principle (OCP)
// The Beverage trait is OPEN for extension (add new beverage types)
// but CLOSED for modification (existing code doesn't change)

use serde::{Deserialize, Serialize};

/// Size of a beverage
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    /// Price multiplier for size
    /// This could be moved to PricingCalculator if size pricing becomes more complex (SRP)
    pub fn price_multiplier(&self) -> f64 {
        match self {
            Size::Small => 0.8,
            Size::Medium => 1.0,
            Size::Large => 1.2,
        }
    }
}

/// SOLID PRINCIPLE: Open-Closed Principle (OCP)
///
/// This trait defines what it means to be a beverage.
/// To add a new beverage type (Latte, Cappuccino, etc.), you:
/// 1. Create a new struct
/// 2. Implement this trait
/// 3. That's it! No changes to OrderService, PricingCalculator, or any other code.
///
/// The system is OPEN for extension (new beverages) but CLOSED for modification.
pub trait Beverage: std::fmt::Debug {
    /// Name of the beverage (e.g., "Espresso", "Green Tea")
    fn name(&self) -> String;

    /// Base price before size adjustment
    fn base_price(&self) -> f64;

    /// Size of the beverage
    fn size(&self) -> Size;

    /// Calculate final price including size
    /// This is a default implementation - beverages can override if needed
    fn price(&self) -> f64 {
        self.base_price() * self.size().price_multiplier()
    }

    /// Description of the beverage
    fn description(&self) -> String {
        format!("{} ({:?})", self.name(), self.size())
    }
}

// ============================================================================
// CONCRETE BEVERAGE IMPLEMENTATIONS
// Each of these is a separate type that implements the Beverage trait
// Adding a new one requires ZERO changes to existing code (OCP)
// ============================================================================

/// A simple coffee
///
/// SOLID: This demonstrates OCP - we can add this type without modifying
/// any existing code. OrderService, PricingCalculator, etc. all work with
/// the Beverage trait, so they automatically support Coffee.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coffee {
    pub size: Size,
    pub extra_shots: u8,
}

impl Beverage for Coffee {
    fn name(&self) -> String {
        if self.extra_shots > 0 {
            format!(
                "Coffee (+{} shot{})",
                self.extra_shots,
                if self.extra_shots > 1 { "s" } else { "" }
            )
        } else {
            "Coffee".to_string()
        }
    }

    fn base_price(&self) -> f64 {
        // Base price + extra shots
        3.50 + (self.extra_shots as f64 * 0.75)
    }

    fn size(&self) -> Size {
        self.size
    }
}

/// A tea beverage
///
/// SOLID: Another OCP example - Tea is a completely independent type
/// that implements Beverage. The rest of the system doesn't care if
/// it's Coffee or Tea; it just works with the Beverage trait.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tea {
    pub size: Size,
    pub variety: String, // "Green", "Black", "Herbal", etc.
}

impl Beverage for Tea {
    fn name(&self) -> String {
        format!("{} Tea", self.variety)
    }

    fn base_price(&self) -> f64 {
        2.50 // Tea is cheaper than coffee
    }

    fn size(&self) -> Size {
        self.size
    }
}

/// A smoothie
///
/// SOLID: Yet another beverage type. Notice how easy it is to add?
/// That's OCP in action. The system is designed for extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Smoothie {
    pub size: Size,
    pub fruits: Vec<String>,
}

impl Beverage for Smoothie {
    fn name(&self) -> String {
        format!("Smoothie ({})", self.fruits.join(", "))
    }

    fn base_price(&self) -> f64 {
        // Base price + extra for each fruit beyond the first
        let base = 5.00;
        let fruit_count = self.fruits.len().max(1) - 1;
        base + (fruit_count as f64 * 0.50)
    }

    fn size(&self) -> Size {
        self.size
    }
}

// ============================================================================
// EXERCISE FOR THE READER:
// Try adding a new beverage type here, like:
// - Latte (with milk type and foam level)
// - Cappuccino (with cinnamon option)
// - Hot Chocolate (with marshmallows)
//
// Notice that you DON'T need to modify:
// - OrderService
// - PricingCalculator
// - Repository
// - Any other existing code!
//
// That's the power of Open-Closed Principle.
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coffee_price() {
        let coffee = Coffee {
            size: Size::Medium,
            extra_shots: 0,
        };

        // Base: 3.50, Medium multiplier: 1.0
        assert_eq!(coffee.price(), 3.50);
    }

    #[test]
    fn test_coffee_with_extra_shots() {
        let coffee = Coffee {
            size: Size::Medium,
            extra_shots: 2,
        };

        // Base: 3.50 + (2 * 0.75) = 5.00, Medium: 1.0
        assert!((coffee.price() - 5.00).abs() < 0.01);
    }

    #[test]
    fn test_tea_price() {
        let tea = Tea {
            size: Size::Large,
            variety: "Green".to_string(),
        };

        // Base: 2.50, Large multiplier: 1.2
        assert!((tea.price() - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_smoothie_price() {
        let smoothie = Smoothie {
            size: Size::Medium,
            fruits: vec!["Strawberry".to_string(), "Banana".to_string()],
        };

        // Base: 5.00 + 0.50 for extra fruit = 5.50, Medium: 1.0
        assert!((smoothie.price() - 5.50).abs() < 0.01);
    }

    #[test]
    fn test_size_multipliers() {
        let coffee = Coffee {
            size: Size::Small,
            extra_shots: 0,
        };

        // Small: 3.50 * 0.8 = 2.80
        assert!((coffee.price() - 2.80).abs() < 0.01);
    }
}
