// SOLID: The Ports Layer (Abstractions)
// 
// This layer defines INTERFACES (traits) that the application needs.
// These are "ports" in Hexagonal Architecture terminology.
// 
// DEPENDENCY INVERSION PRINCIPLE (DIP):
// 
// This layer is the KEY to DIP:
// - Domain layer (inner) has no dependencies
// - Ports layer defines interfaces needed by the application
// - Services layer depends on these interfaces
// - Adapters layer implements these interfaces
// 
// The dependency flow:
// Domain <-- Ports <-- Services
//             ^
//             |
//          Adapters
// 
// Everything depends on abstractions (traits in this layer).
// No component depends on concrete implementations.
// 
// Benefits:
// 1. Business logic is independent of infrastructure
// 2. We can swap implementations without changing business logic
// 3. Testing is easy (use mock implementations)
// 4. Different teams can work on adapters independently

pub mod notifier;
pub mod payment;
pub mod repository;

// Re-export for convenience
pub use notifier::{NotificationError, Notifier};
pub use payment::{PaymentError, PaymentProcessor};
pub use repository::{OrderRepository, RepositoryError};
