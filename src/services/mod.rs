//! Service modules for Appwrite SDK

pub mod account;
pub use account::Account;
pub mod activities;
pub use activities::Activities;
pub mod avatars;
pub use avatars::Avatars;
pub mod backups;
pub use backups::Backups;
pub mod databases;
pub use databases::Databases;
pub mod functions;
pub use functions::Functions;
pub mod graphql;
pub use graphql::Graphql;
pub mod health;
pub use health::Health;
pub mod locale;
pub use locale::Locale;
pub mod messaging;
pub use messaging::Messaging;
pub mod project;
pub use project::Project;
pub mod sites;
pub use sites::Sites;
pub mod storage;
pub use storage::Storage;
pub mod tables_db;
pub use tables_db::TablesDB;
pub mod teams;
pub use teams::Teams;
pub mod tokens;
pub use tokens::Tokens;
pub mod users;
pub use users::Users;
pub mod webhooks;
pub use webhooks::Webhooks;

use crate::client::Client;

/// Base trait for all Appwrite services
pub trait Service {
    /// Get a reference to the client
    fn client(&self) -> &Client;
}

// Re-export all services for convenience
pub struct Services {
    client: Client,
    account: Account,
    activities: Activities,
    avatars: Avatars,
    backups: Backups,
    databases: Databases,
    functions: Functions,
    graphql: Graphql,
    health: Health,
    locale: Locale,
    messaging: Messaging,
    project: Project,
    sites: Sites,
    storage: Storage,
    tables_db: TablesDB,
    teams: Teams,
    tokens: Tokens,
    users: Users,
    webhooks: Webhooks,
}

impl Services {
    /// Create new services instance
    pub fn new(client: Client) -> Self {
        Self {
            account: Account::new(&client),
            activities: Activities::new(&client),
            avatars: Avatars::new(&client),
            backups: Backups::new(&client),
            databases: Databases::new(&client),
            functions: Functions::new(&client),
            graphql: Graphql::new(&client),
            health: Health::new(&client),
            locale: Locale::new(&client),
            messaging: Messaging::new(&client),
            project: Project::new(&client),
            sites: Sites::new(&client),
            storage: Storage::new(&client),
            tables_db: TablesDB::new(&client),
            teams: Teams::new(&client),
            tokens: Tokens::new(&client),
            users: Users::new(&client),
            webhooks: Webhooks::new(&client),
            client,
        }
    }

    /// Get client reference
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get Account service
    pub fn account(&self) -> &Account {
        &self.account
    }
    /// Get Activities service
    pub fn activities(&self) -> &Activities {
        &self.activities
    }
    /// Get Avatars service
    pub fn avatars(&self) -> &Avatars {
        &self.avatars
    }
    /// Get Backups service
    pub fn backups(&self) -> &Backups {
        &self.backups
    }
    /// Get Databases service
    pub fn databases(&self) -> &Databases {
        &self.databases
    }
    /// Get Functions service
    pub fn functions(&self) -> &Functions {
        &self.functions
    }
    /// Get Graphql service
    pub fn graphql(&self) -> &Graphql {
        &self.graphql
    }
    /// Get Health service
    pub fn health(&self) -> &Health {
        &self.health
    }
    /// Get Locale service
    pub fn locale(&self) -> &Locale {
        &self.locale
    }
    /// Get Messaging service
    pub fn messaging(&self) -> &Messaging {
        &self.messaging
    }
    /// Get Project service
    pub fn project(&self) -> &Project {
        &self.project
    }
    /// Get Sites service
    pub fn sites(&self) -> &Sites {
        &self.sites
    }
    /// Get Storage service
    pub fn storage(&self) -> &Storage {
        &self.storage
    }
    /// Get TablesDB service
    pub fn tables_db(&self) -> &TablesDB {
        &self.tables_db
    }
    /// Get Teams service
    pub fn teams(&self) -> &Teams {
        &self.teams
    }
    /// Get Tokens service
    pub fn tokens(&self) -> &Tokens {
        &self.tokens
    }
    /// Get Users service
    pub fn users(&self) -> &Users {
        &self.users
    }
    /// Get Webhooks service
    pub fn webhooks(&self) -> &Webhooks {
        &self.webhooks
    }
}
