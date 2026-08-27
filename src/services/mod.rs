//! Service modules for Appwrite SDK

pub mod account;
pub use account::Account;
pub mod activities;
pub use activities::Activities;
pub mod apps;
pub use apps::Apps;
pub mod avatars;
pub use avatars::Avatars;
pub mod backups;
pub use backups::Backups;
pub mod databases;
pub use databases::Databases;
pub mod documents_db;
pub use documents_db::DocumentsDB;
pub mod embeddings;
pub use embeddings::Embeddings;
pub mod functions;
pub use functions::Functions;
pub mod graphql;
pub use graphql::Graphql;
pub mod locale;
pub use locale::Locale;
pub mod messaging;
pub use messaging::Messaging;
pub mod mongo;
pub use mongo::Mongo;
pub mod mysql;
pub use mysql::Mysql;
pub mod oauth2;
pub use oauth2::Oauth2;
pub mod organization;
pub use organization::Organization;
pub mod postgresql;
pub use postgresql::Postgresql;
pub mod presences;
pub use presences::Presences;
pub mod project;
pub use project::Project;
pub mod proxy;
pub use proxy::Proxy;
pub mod advisor;
pub use advisor::Advisor;
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
pub mod vectors_db;
pub use vectors_db::VectorsDB;
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
    apps: Apps,
    avatars: Avatars,
    backups: Backups,
    databases: Databases,
    documents_db: DocumentsDB,
    embeddings: Embeddings,
    functions: Functions,
    graphql: Graphql,
    locale: Locale,
    messaging: Messaging,
    mongo: Mongo,
    mysql: Mysql,
    oauth2: Oauth2,
    organization: Organization,
    postgresql: Postgresql,
    presences: Presences,
    project: Project,
    proxy: Proxy,
    advisor: Advisor,
    sites: Sites,
    storage: Storage,
    tables_db: TablesDB,
    teams: Teams,
    tokens: Tokens,
    users: Users,
    vectors_db: VectorsDB,
    webhooks: Webhooks,
}

impl Services {
    /// Create new services instance
    pub fn new(client: Client) -> Self {
        Self {
            account: Account::new(&client),
            activities: Activities::new(&client),
            apps: Apps::new(&client),
            avatars: Avatars::new(&client),
            backups: Backups::new(&client),
            databases: Databases::new(&client),
            documents_db: DocumentsDB::new(&client),
            embeddings: Embeddings::new(&client),
            functions: Functions::new(&client),
            graphql: Graphql::new(&client),
            locale: Locale::new(&client),
            messaging: Messaging::new(&client),
            mongo: Mongo::new(&client),
            mysql: Mysql::new(&client),
            oauth2: Oauth2::new(&client),
            organization: Organization::new(&client),
            postgresql: Postgresql::new(&client),
            presences: Presences::new(&client),
            project: Project::new(&client),
            proxy: Proxy::new(&client),
            advisor: Advisor::new(&client),
            sites: Sites::new(&client),
            storage: Storage::new(&client),
            tables_db: TablesDB::new(&client),
            teams: Teams::new(&client),
            tokens: Tokens::new(&client),
            users: Users::new(&client),
            vectors_db: VectorsDB::new(&client),
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
    /// Get Apps service
    pub fn apps(&self) -> &Apps {
        &self.apps
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
    /// Get DocumentsDB service
    pub fn documents_db(&self) -> &DocumentsDB {
        &self.documents_db
    }
    /// Get Embeddings service
    pub fn embeddings(&self) -> &Embeddings {
        &self.embeddings
    }
    /// Get Functions service
    pub fn functions(&self) -> &Functions {
        &self.functions
    }
    /// Get Graphql service
    pub fn graphql(&self) -> &Graphql {
        &self.graphql
    }
    /// Get Locale service
    pub fn locale(&self) -> &Locale {
        &self.locale
    }
    /// Get Messaging service
    pub fn messaging(&self) -> &Messaging {
        &self.messaging
    }
    /// Get Mongo service
    pub fn mongo(&self) -> &Mongo {
        &self.mongo
    }
    /// Get Mysql service
    pub fn mysql(&self) -> &Mysql {
        &self.mysql
    }
    /// Get Oauth2 service
    pub fn oauth2(&self) -> &Oauth2 {
        &self.oauth2
    }
    /// Get Organization service
    pub fn organization(&self) -> &Organization {
        &self.organization
    }
    /// Get Postgresql service
    pub fn postgresql(&self) -> &Postgresql {
        &self.postgresql
    }
    /// Get Presences service
    pub fn presences(&self) -> &Presences {
        &self.presences
    }
    /// Get Project service
    pub fn project(&self) -> &Project {
        &self.project
    }
    /// Get Proxy service
    pub fn proxy(&self) -> &Proxy {
        &self.proxy
    }
    /// Get Advisor service
    pub fn advisor(&self) -> &Advisor {
        &self.advisor
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
    /// Get VectorsDB service
    pub fn vectors_db(&self) -> &VectorsDB {
        &self.vectors_db
    }
    /// Get Webhooks service
    pub fn webhooks(&self) -> &Webhooks {
        &self.webhooks
    }
}
