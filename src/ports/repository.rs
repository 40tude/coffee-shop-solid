// SOLID: This module defines the Repository PORT (abstraction)
// 
// DEPENDENCY INVERSION PRINCIPLE (DIP):
// - High-level business logic (OrderService) depends on THIS TRAIT
// - Low-level implementations (MemoryStorage, JsonStorage) also depend on THIS TRAIT
// - Dependencies point INWARD toward the abstraction
//
// This means:
// - OrderService doesn't know about JSON files or databases
// - We can swap storage implementations without changing business logic
// - We can test with in-memory storage (no files/databases needed)

use crate::domain::Order;
use std::error::Error;
use std::fmt;

/// Error type for repository operations
/// 
/// SOLID: This error type is part of the contract.
/// All implementations must use this error type, ensuring they're
/// substitutable (Liskov Substitution Principle - LSP).
#[derive(Debug, Clone)]
pub enum RepositoryError {
    NotFound(String),
    SaveFailed(String),
    LoadFailed(String),
    AlreadyExists(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RepositoryError::NotFound(msg) => write!(f, "Not found: {}", msg),
            RepositoryError::SaveFailed(msg) => write!(f, "Save failed: {}", msg),
            RepositoryError::LoadFailed(msg) => write!(f, "Load failed: {}", msg),
            RepositoryError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
        }
    }
}

impl Error for RepositoryError {}

/// Repository trait for storing and retrieving orders
/// 
/// SOLID PRINCIPLES DEMONSTRATED:
/// 
/// 1. DEPENDENCY INVERSION PRINCIPLE (DIP):
///    - This is the ABSTRACTION that high-level code depends on
///    - Concrete implementations (Memory, JSON, Postgres) depend on this same abstraction
///    - The dependency arrow points toward this trait, not toward implementations
/// 
/// 2. INTERFACE SEGREGATION PRINCIPLE (ISP):
///    - This trait is FOCUSED on storage operations only
///    - It doesn't have methods for payment, notification, pricing, etc.
///    - Clients that only need storage depend only on storage operations
/// 
/// 3. OPEN-CLOSED PRINCIPLE (OCP):
///    - To add a new storage backend (e.g., PostgreSQL), just implement this trait
///    - No changes needed to OrderService or any other existing code
/// 
/// 4. LISKOV SUBSTITUTION PRINCIPLE (LSP):
///    - Any implementation of this trait should be substitutable
///    - All implementations must honor the same contract (return types, error semantics)
///    - A user of OrderRepository shouldn't care if it's Memory, JSON, or Postgres
pub trait OrderRepository {
    /// Save an order
    /// 
    /// Contract: 
    /// - If order.id already exists, return RepositoryError::AlreadyExists
    /// - If save succeeds, return Ok(())
    /// - If save fails for any other reason, return RepositoryError::SaveFailed
    fn save(&mut self, order: &Order) -> Result<(), RepositoryError>;

    /// Find an order by ID
    /// 
    /// Contract:
    /// - If order exists, return Ok(Some(order))
    /// - If order doesn't exist, return Ok(None)
    /// - If retrieval fails, return RepositoryError::LoadFailed
    fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Order>, RepositoryError>;

    /// Find all orders for a customer
    /// 
    /// Contract:
    /// - Returns all orders for the customer (can be empty vec)
    /// - If retrieval fails, return RepositoryError::LoadFailed
    fn find_by_customer_email(&self, email: &str) -> Result<Vec<Order>, RepositoryError>;

    /// List all orders
    /// 
    /// Contract:
    /// - Returns all orders in the system (can be empty vec)
    /// - If retrieval fails, return RepositoryError::LoadFailed
    fn list_all(&self) -> Result<Vec<Order>, RepositoryError>;

    /// Update an existing order
    /// 
    /// Contract:
    /// - If order doesn't exist, return RepositoryError::NotFound
    /// - If update succeeds, return Ok(())
    /// - If update fails, return RepositoryError::SaveFailed
    fn update(&mut self, order: &Order) -> Result<(), RepositoryError>;

    /// Delete an order by ID
    /// 
    /// Contract:
    /// - If order exists and is deleted, return Ok(true)
    /// - If order doesn't exist, return Ok(false)
    /// - If deletion fails, return RepositoryError::SaveFailed
    fn delete(&mut self, id: uuid::Uuid) -> Result<bool, RepositoryError>;
}

// ============================================================================
// KEY INSIGHT: The Repository Pattern + DIP
// 
// The Repository pattern is a perfect example of DIP:
// 
// WITHOUT DIP (bad):
// OrderService --> PostgresOrderRepository --> postgres crate
//   (high-level)       (low-level)              (external)
// 
// WITH DIP (good):
// OrderService --> OrderRepository <-- PostgresOrderRepository
//   (high-level)    (abstraction)         (low-level)
// 
// Benefits:
// 1. OrderService has ZERO knowledge of storage implementation
// 2. We can test with MemoryOrderRepository (no database needed)
// 3. We can swap PostgreSQL for MongoDB without touching OrderService
// 4. Multiple teams can work on different storage implementations
// 5. Business logic is independent of infrastructure
// ============================================================================
