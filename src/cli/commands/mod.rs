pub mod backup;
pub mod bucket;
pub mod cluster;
pub mod health;
pub mod key;
pub mod metrics;
pub mod server;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum BucketCommands {
    /// Create a new bucket
    Create {
        /// Bucket name
        name: String,
        /// Make bucket public
        #[arg(long)]
        public: bool,
    },
    /// List all buckets
    List,
    /// Delete a bucket
    Delete {
        /// Bucket name
        name: String,
        /// Force delete even if not empty
        #[arg(long)]
        force: bool,
    },
    /// Show bucket details
    Info {
        /// Bucket name
        name: String,
    },
}

impl BucketCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::Create { name, public } => bucket::create(name, public).await,
            Self::List => bucket::list().await,
            Self::Delete { name, force } => bucket::delete(name, force).await,
            Self::Info { name } => bucket::info(name).await,
        }
    }
}

#[derive(Subcommand)]
pub enum KeyCommands {
    /// Create a new access key
    Create {
        /// Key name/description
        #[arg(long)]
        name: String,
        /// Read-only access
        #[arg(long)]
        read_only: bool,
        /// Read-write access
        #[arg(long)]
        read_write: bool,
    },
    /// List all access keys
    List,
    /// Revoke an access key
    Revoke {
        /// Access key ID
        key_id: String,
    },
    /// Show key details
    Info {
        /// Access key ID
        key_id: String,
    },
}

impl KeyCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::Create {
                name,
                read_only,
                read_write,
            } => key::create(name, read_only, read_write).await,
            Self::List => key::list().await,
            Self::Revoke { key_id } => key::revoke(key_id).await,
            Self::Info { key_id } => key::info(key_id).await,
        }
    }
}

#[derive(Subcommand)]
pub enum ClusterCommands {
    /// Show cluster status
    Status,
    /// Add a new node
    AddNode {
        /// Node IP address
        #[arg(long)]
        ip: String,
        /// Node port
        #[arg(long, default_value = "3900")]
        port: u16,
    },
    /// Remove a node
    RemoveNode {
        /// Node ID
        node_id: String,
    },
    /// Rebalance cluster data
    Rebalance,
}

impl ClusterCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::Status => cluster::status().await,
            Self::AddNode { ip, port } => cluster::add_node(ip, port).await,
            Self::RemoveNode { node_id } => cluster::remove_node(node_id).await,
            Self::Rebalance => cluster::rebalance().await,
        }
    }
}

#[derive(Subcommand)]
pub enum BackupCommands {
    /// Create a backup
    Create {
        /// Backup name
        #[arg(long)]
        name: String,
        /// Backup destination
        #[arg(long)]
        destination: String,
    },
    /// List all backups
    List,
    /// Restore from backup
    Restore {
        /// Backup ID
        #[arg(long)]
        id: u64,
    },
}

impl BackupCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::Create { name, destination } => backup::create(name, destination).await,
            Self::List => backup::list().await,
            Self::Restore { id } => backup::restore(id).await,
        }
    }
}
