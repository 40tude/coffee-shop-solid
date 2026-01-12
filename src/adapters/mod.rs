// SOLID: The Adapters Layer (Implementations)
// 
// This layer contains concrete implementations of the ports (traits).
// These are "adapters" in Hexagonal Architecture terminology.
// 
// PRINCIPLES:
// 
// 1. DEPENDENCY INVERSION PRINCIPLE (DIP):
//    Adapters depend on ports (traits), not the other way around.
//    The dependency arrow points INWARD toward abstractions.
// 
// 2. OPEN-CLOSED PRINCIPLE (OCP):
//    We can add new adapters (new storage, payment, notification methods)
//    without modifying existing code.
//    Each adapter is independent.
// 
// 3. LISKOV SUBSTITUTION PRINCIPLE (LSP):
//    All adapters implementing the same trait must be substitutable.
//    OrderService shouldn't care which implementation it receives.
// 
// 4. INTERFACE SEGREGATION PRINCIPLE (ISP):
//    Each adapter implements only the focused interfaces it needs.
//    A notifier doesn't implement storage or payment interfaces.
// 
// STRUCTURE:
// - Storage adapters: MemoryOrderRepository, JsonOrderRepository
// - Payment adapters: CashPayment, CreditCardPayment
// - Notification adapters: ConsoleNotifier
// 
// ADDING NEW ADAPTERS:
// Want to add PostgreSQL storage? Create postgres_storage.rs and implement OrderRepository.
// Want to add PayPal payment? Create paypal_payment.rs and implement PaymentProcessor.
// Want to add SMS notifications? Create sms_notifier.rs and implement Notifier.
// 
// No changes needed to:
// - Domain layer
// - Ports layer
// - Services layer
// - Existing adapters
// 
// That's SOLID in action!

pub mod cash_payment;
pub mod console_notifier;
pub mod credit_card_payment;
pub mod json_storage;
pub mod memory_storage;

// Re-export for convenience
pub use cash_payment::CashPayment;
pub use console_notifier::ConsoleNotifier;
pub use credit_card_payment::CreditCardPayment;
pub use json_storage::JsonOrderRepository;
pub use memory_storage::MemoryOrderRepository;
