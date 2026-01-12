// SOLID: The Domain Layer
// 
// This layer contains pure business entities with NO external dependencies.
// Following the Dependency Inversion Principle (DIP), this is the CORE
// of our application. Everything else depends on this, but this depends on nothing.
//
// Domain entities:
// - Have no knowledge of databases, HTTP, JSON, or any infrastructure
// - Contain business rules and data structures
// - Are easy to test (no mocks needed)
// - Can be understood without reading any other code

pub mod beverage;
pub mod customer;
pub mod order;

// Re-export commonly used types for convenience
pub use beverage::{Beverage, Coffee, Size, Smoothie, Tea};
pub use customer::Customer;
pub use order::{Order, OrderItem, OrderStatus};
