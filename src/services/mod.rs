// SOLID: The Services Layer (Application Logic)
// 
// This layer contains business logic that coordinates domain entities
// and uses ports (abstractions) to interact with infrastructure.
// 
// PRINCIPLES:
// 
// 1. SINGLE RESPONSIBILITY PRINCIPLE (SRP):
//    Each service has ONE responsibility:
//    - OrderService: manage order workflow
//    - PricingCalculator: calculate prices
//    
//    If we added more services:
//    - InventoryService: manage beverage inventory
//    - ReportingService: generate reports
//    
//    Each would have its own file and single responsibility.
// 
// 2. DEPENDENCY INVERSION PRINCIPLE (DIP):
//    Services depend on ports (traits), not on adapters (implementations).
//    This allows us to swap implementations without changing business logic.

pub mod order_service;
pub mod pricing_calculator;

// Re-export for convenience
pub use order_service::{OrderService, OrderServiceError};
pub use pricing_calculator::PricingCalculator;
