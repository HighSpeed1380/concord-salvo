use crate::models::server::{FieldsRole, FieldsServer, PartialRole, PartialServer, Role, Server};
use crate::Result;

#[async_trait]
pub trait AbstractServer: Sync + Send {
    /// Fetch a server by its id
    async fn fetch_server(&self, id: &str) -> Result<Server>;

    /// Fetch a servers by their ids
    async fn fetch_servers<'a>(&self, ids: &'a [String]) -> Result<Vec<Server>>;

    /// Insert a new server into database
    async fn insert_server(&self, server: &Server) -> Result<()>;

    /// Update a server with new information
    async fn update_server(
        &self,
        id: &str,
        server: &PartialServer,
        remove: Vec<FieldsServer>,
    ) -> Result<()>;

    /// Delete a server by its id
    async fn delete_server(&self, server: &Server) -> Result<()>;

    /// Insert a new role into server object
    async fn insert_role(&self, server_id: &str, role_id: &str, role: &Role) -> Result<()>;

    /// Update an existing role on a server
    async fn update_role(
        &self,
        server_id: &str,
        role_id: &str,
        role: &PartialRole,
        remove: Vec<FieldsRole>,
    ) -> Result<()>;

    /// Delete a role from a server
    ///
    /// Also updates channels and members.
    async fn delete_role(&self, server_id: &str, role_id: &str) -> Result<()>;
}
