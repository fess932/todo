pub mod models;
pub mod store;

/// This is the service definition. It looks a lot like a trait definition.
/// It defines one RPC, hello, which takes one arg, name, and returns a String.
#[tarpc::service]
pub trait World {
    /// Returns a greeting for name.
    async fn new_task(task: models::NewTask) -> models::Task;
}
