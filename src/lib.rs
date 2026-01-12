// Coffee Shop Order System - Demonstrating SOLID Principles in Rust
// 
// This library demonstrates all five SOLID principles in action:
// 
// 1. SINGLE RESPONSIBILITY PRINCIPLE (SRP):
//    - Each module has one reason to change
//    - PricingCalculator: owned by Accounting
//    - OrderService: owned by Operations
//    - OrderRepository: owned by DBAs
//    - Notifier: owned by Customer Service
// 
// 2. OPEN-CLOSED PRINCIPLE (OCP):
//    - Add new beverages without modifying existing code
//    - Add new payment methods without modifying OrderService
//    - Add new storage backends without modifying business logic
// 
// 3. LISKOV SUBSTITUTION PRINCIPLE (LSP):
//    - Any OrderRepository implementation is substitutable
//    - Any PaymentProcessor implementation is substitutable
//    - Any Notifier implementation is substitutable
// 
// 4. INTERFACE SEGREGATION PRINCIPLE (ISP):
//    - Small, focused traits (Notifier, PaymentProcessor, OrderRepository)
//    - No fat interfaces with dozens of methods
//    - Each component depends only on what it needs
// 
// 5. DEPENDENCY INVERSION PRINCIPLE (DIP):
//    - High-level modules (OrderService) depend on abstractions (traits)
//    - Low-level modules (adapters) depend on the same abstractions
//    - Dependencies point inward toward abstractions, not outward toward details
// 
// LAYER STRUCTURE:
// ```
// Domain (pure entities, no dependencies)
//   ↑
// Ports (trait definitions, abstractions)
//   ↑
// Services (business logic, depends on ports)
//   ↑
// Adapters (implementations, depends on ports)
// ```
// 
// Everything depends on abstractions. Nothing depends on concrete details.
// This is the essence of Clean Architecture + SOLID principles.

// Domain layer - pure business entities
pub mod domain;

// Ports layer - trait definitions (abstractions)
pub mod ports;

// Services layer - business logic
pub mod services;

// Adapters layer - concrete implementations
pub mod adapters;

// Re-export commonly used types for convenience
pub use domain::{Beverage, Coffee, Customer, Order, OrderItem, Size, Smoothie, Tea};
pub use ports::{Notifier, OrderRepository, PaymentProcessor};
pub use services::{OrderService, PricingCalculator};
pub use adapters::{CashPayment, ConsoleNotifier, MemoryOrderRepository};

// ============================================================================
// QUICK START EXAMPLE
// 
// ```rust
// use coffee_shop_solid::*;
// 
// // Create dependencies (dependency injection)
// let repository = MemoryOrderRepository::new();
// let payment = CashPayment;
// let notifier = ConsoleNotifier;
// 
// // Create service (depends on abstractions, not concretions)
// let mut service = OrderService::new(repository, payment, notifier);
// 
// // Create customer
// let customer = Customer::new(
//     "Alice".to_string(),
//     "alice@example.com".to_string(),
//     None,
// );
// 
// // Create order
// let beverages: Vec<Box<dyn Beverage>> = vec![
//     Box::new(Coffee {
//         size: Size::Large,
//         extra_shots: 1,
//     }),
// ];
// 
// // Place order
// let order = service.place_order(customer, beverages).unwrap();
// println!("Order placed: {}", order.id);
// ```
// ============================================================================

// ============================================================================
// SOLID BENEFITS IN THIS CODEBASE
// 
// 1. EASY TO TEST:
//    - Use MemoryOrderRepository for fast tests (no file I/O)
//    - Use mock implementations for any trait
//    - Business logic is independent of infrastructure
// 
// 2. EASY TO EXTEND:
//    - Add new beverage types in domain/beverage.rs (OCP)
//    - Add new payment methods in adapters/ (OCP)
//    - Add new storage backends in adapters/ (OCP)
//    - No changes needed to existing code
// 
// 3. EASY TO MAINTAIN:
//    - Each module has one responsibility (SRP)
//    - Changes are localized (pricing rules only in PricingCalculator)
//    - No ripple effects across the system
// 
// 4. EASY TO SWAP:
//    - Swap MemoryOrderRepository for JsonOrderRepository (DIP)
//    - Swap CashPayment for CreditCardPayment (DIP)
//    - Swap ConsoleNotifier for EmailNotifier (DIP)
//    - Business logic unchanged
// 
// 5. EASY TO UNDERSTAND:
//    - Small, focused interfaces (ISP)
//    - Clear dependencies (DIP)
//    - Each module has a single purpose (SRP)
// ============================================================================
