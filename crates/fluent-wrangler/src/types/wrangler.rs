// Builder imports — uncomment as each ops module is implemented:
// use crate::ops::account::WhoAmIBuilder;
// use crate::ops::worker::{DeployBuilder, DeleteBuilder, RollbackBuilder};
// use crate::ops::deployment::{DeploymentsListBuilder, DeploymentsStatusBuilder};
// use crate::ops::version::{VersionsListBuilder, VersionsViewBuilder, VersionsUploadBuilder, VersionsDeployBuilder};
// use crate::ops::secret::{SecretListBuilder, SecretPutBuilder, SecretDeleteBuilder, SecretBulkBuilder};
// use crate::ops::d1::{D1ListBuilder, D1InfoBuilder, D1CreateBuilder, D1DeleteBuilder, D1ExecuteBuilder};
// use crate::ops::d1_migration::{D1MigrationsCreateBuilder, D1MigrationsListBuilder, D1MigrationsApplyBuilder};
// use crate::ops::d1_time_travel::{D1TimeTravelInfoBuilder, D1TimeTravelRestoreBuilder};
// use crate::ops::kv_namespace::{KvNamespaceCreateBuilder, KvNamespaceListBuilder, KvNamespaceDeleteBuilder, KvNamespaceRenameBuilder};
// use crate::ops::kv_key::{KvKeyPutBuilder, KvKeyGetBuilder, KvKeyListBuilder, KvKeyDeleteBuilder};
// use crate::ops::kv_bulk::{KvBulkGetBuilder, KvBulkPutBuilder, KvBulkDeleteBuilder};
// use crate::ops::r2_bucket::{R2BucketCreateBuilder, R2BucketListBuilder, R2BucketInfoBuilder, R2BucketDeleteBuilder};
// use crate::ops::r2_object::{R2ObjectGetBuilder, R2ObjectPutBuilder, R2ObjectDeleteBuilder};
// use crate::ops::pages::{PagesProjectListBuilder, PagesProjectCreateBuilder, PagesProjectDeleteBuilder, PagesDeployBuilder, PagesDeploymentListBuilder, PagesDeploymentDeleteBuilder};
// use crate::ops::queues::{QueuesListBuilder, QueuesCreateBuilder, QueuesUpdateBuilder, QueuesDeleteBuilder, QueuesInfoBuilder, QueuesConsumerAddBuilder, QueuesConsumerRemoveBuilder, QueuesPauseDeliveryBuilder, QueuesResumeDeliveryBuilder, QueuesPurgeBuilder};
// use crate::ops::vectorize::{VectorizeListBuilder, VectorizeCreateBuilder, VectorizeDeleteBuilder, VectorizeGetBuilder, VectorizeInfoBuilder, VectorizeInsertBuilder, VectorizeUpsertBuilder, VectorizeQueryBuilder, VectorizeGetVectorsBuilder, VectorizeDeleteVectorsBuilder};
// use crate::ops::hyperdrive::{HyperdriveListBuilder, HyperdriveCreateBuilder, HyperdriveDeleteBuilder, HyperdriveGetBuilder, HyperdriveUpdateBuilder};
// use crate::ops::workflows::{WorkflowsListBuilder, WorkflowsDescribeBuilder, WorkflowsDeleteBuilder, WorkflowsTriggerBuilder, WorkflowsInstancesListBuilder, WorkflowsInstancesDescribeBuilder, WorkflowsInstancesTerminateBuilder, WorkflowsInstancesPauseBuilder, WorkflowsInstancesResumeBuilder};

/// Entry point for Cloudflare Wrangler CLI operations.
///
/// Stateless — configuration is per-invocation.
#[derive(Debug, Clone, Default)]
pub struct Wrangler;

impl Wrangler {
    pub fn new() -> Self {
        Self
    }

    // ── Account ─────────────────────────────────────────────────────

    // pub fn whoami(&self) -> WhoAmIBuilder { WhoAmIBuilder::new() }

    // ── Workers ─────────────────────────────────────────────────────

    // pub fn deploy(&self) -> DeployBuilder { DeployBuilder::new() }
    // pub fn delete(&self, name: impl Into<String>) -> DeleteBuilder { DeleteBuilder::new(name) }
    // pub fn rollback(&self) -> RollbackBuilder { RollbackBuilder::new() }

    // ── Groups ──────────────────────────────────────────────────────

    /// Access deployment sub-commands.
    pub fn deployments(&self) -> DeploymentsGroup {
        DeploymentsGroup
    }

    /// Access version sub-commands.
    pub fn versions(&self) -> VersionsGroup {
        VersionsGroup
    }

    /// Access secret sub-commands.
    pub fn secret(&self) -> SecretGroup {
        SecretGroup
    }

    /// Access D1 database sub-commands.
    pub fn d1(&self) -> D1Group {
        D1Group
    }

    /// Access KV sub-commands.
    pub fn kv(&self) -> KvGroup {
        KvGroup
    }

    /// Access R2 sub-commands.
    pub fn r2(&self) -> R2Group {
        R2Group
    }

    /// Access Pages sub-commands.
    pub fn pages(&self) -> PagesGroup {
        PagesGroup
    }

    /// Access Queues sub-commands.
    pub fn queues(&self) -> QueuesGroup {
        QueuesGroup
    }

    /// Access Vectorize sub-commands.
    pub fn vectorize(&self) -> VectorizeGroup {
        VectorizeGroup
    }

    /// Access Hyperdrive sub-commands.
    pub fn hyperdrive(&self) -> HyperdriveGroup {
        HyperdriveGroup
    }

    /// Access Workflows sub-commands.
    pub fn workflows(&self) -> WorkflowsGroup {
        WorkflowsGroup
    }
}

impl fluent_core::tool::CliTool for Wrangler {
    fn program() -> &'static str {
        "wrangler"
    }
}

// ── Deployments Group ───────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DeploymentsGroup;

// impl DeploymentsGroup {
//     pub fn list(&self) -> DeploymentsListBuilder { DeploymentsListBuilder::new() }
//     pub fn status(&self) -> DeploymentsStatusBuilder { DeploymentsStatusBuilder::new() }
// }

// ── Versions Group ──────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct VersionsGroup;

// impl VersionsGroup {
//     pub fn list(&self) -> VersionsListBuilder { VersionsListBuilder::new() }
//     pub fn view(&self, version_id: impl Into<String>) -> VersionsViewBuilder { VersionsViewBuilder::new(version_id) }
//     pub fn upload(&self) -> VersionsUploadBuilder { VersionsUploadBuilder::new() }
//     pub fn deploy(&self) -> VersionsDeployBuilder { VersionsDeployBuilder::new() }
// }

// ── Secret Group ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SecretGroup;

// impl SecretGroup {
//     pub fn list(&self) -> SecretListBuilder { SecretListBuilder::new() }
//     pub fn put(&self, key: impl Into<String>) -> SecretPutBuilder { SecretPutBuilder::new(key) }
//     pub fn delete(&self, key: impl Into<String>) -> SecretDeleteBuilder { SecretDeleteBuilder::new(key) }
//     pub fn bulk(&self) -> SecretBulkBuilder { SecretBulkBuilder::new() }
// }

// ── D1 Group ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct D1Group;

// impl D1Group {
//     pub fn list(&self) -> D1ListBuilder { D1ListBuilder::new() }
//     pub fn info(&self, name: impl Into<String>) -> D1InfoBuilder { D1InfoBuilder::new(name) }
//     pub fn create(&self, name: impl Into<String>) -> D1CreateBuilder { D1CreateBuilder::new(name) }
//     pub fn delete(&self, name: impl Into<String>) -> D1DeleteBuilder { D1DeleteBuilder::new(name) }
//     pub fn execute(&self, database: impl Into<String>) -> D1ExecuteBuilder { D1ExecuteBuilder::new(database) }
//     pub fn migrations(&self) -> D1MigrationsGroup { D1MigrationsGroup }
//     pub fn time_travel(&self) -> D1TimeTravelGroup { D1TimeTravelGroup }
// }

// ── D1 Migrations Group ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct D1MigrationsGroup;

// impl D1MigrationsGroup {
//     pub fn create(&self, database: impl Into<String>, message: impl Into<String>) -> D1MigrationsCreateBuilder { D1MigrationsCreateBuilder::new(database, message) }
//     pub fn list(&self, database: impl Into<String>) -> D1MigrationsListBuilder { D1MigrationsListBuilder::new(database) }
//     pub fn apply(&self, database: impl Into<String>) -> D1MigrationsApplyBuilder { D1MigrationsApplyBuilder::new(database) }
// }

// ── D1 Time Travel Group ────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct D1TimeTravelGroup;

// impl D1TimeTravelGroup {
//     pub fn info(&self, database: impl Into<String>) -> D1TimeTravelInfoBuilder { D1TimeTravelInfoBuilder::new(database) }
//     pub fn restore(&self, database: impl Into<String>) -> D1TimeTravelRestoreBuilder { D1TimeTravelRestoreBuilder::new(database) }
// }

// ── KV Group ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct KvGroup;

impl KvGroup {
    /// Access KV namespace sub-commands.
    pub fn namespace(&self) -> KvNamespaceGroup {
        KvNamespaceGroup
    }

    /// Access KV key sub-commands.
    pub fn key(&self) -> KvKeyGroup {
        KvKeyGroup
    }

    /// Access KV bulk sub-commands.
    pub fn bulk(&self) -> KvBulkGroup {
        KvBulkGroup
    }
}

// ── KV Namespace Group ──────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct KvNamespaceGroup;

// impl KvNamespaceGroup {
//     pub fn create(&self, title: impl Into<String>) -> KvNamespaceCreateBuilder { KvNamespaceCreateBuilder::new(title) }
//     pub fn list(&self) -> KvNamespaceListBuilder { KvNamespaceListBuilder::new() }
//     pub fn delete(&self) -> KvNamespaceDeleteBuilder { KvNamespaceDeleteBuilder::new() }
//     pub fn rename(&self, old_namespace: impl Into<String>, new_title: impl Into<String>) -> KvNamespaceRenameBuilder { KvNamespaceRenameBuilder::new(old_namespace, new_title) }
// }

// ── KV Key Group ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct KvKeyGroup;

// impl KvKeyGroup {
//     pub fn put(&self, key: impl Into<String>) -> KvKeyPutBuilder { KvKeyPutBuilder::new(key) }
//     pub fn get(&self, key: impl Into<String>) -> KvKeyGetBuilder { KvKeyGetBuilder::new(key) }
//     pub fn list(&self) -> KvKeyListBuilder { KvKeyListBuilder::new() }
//     pub fn delete(&self, key: impl Into<String>) -> KvKeyDeleteBuilder { KvKeyDeleteBuilder::new(key) }
// }

// ── KV Bulk Group ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct KvBulkGroup;

// impl KvBulkGroup {
//     pub fn get(&self, file: impl Into<String>) -> KvBulkGetBuilder { KvBulkGetBuilder::new(file) }
//     pub fn put(&self, file: impl Into<String>) -> KvBulkPutBuilder { KvBulkPutBuilder::new(file) }
//     pub fn delete(&self, file: impl Into<String>) -> KvBulkDeleteBuilder { KvBulkDeleteBuilder::new(file) }
// }

// ── R2 Group ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct R2Group;

impl R2Group {
    /// Access R2 bucket sub-commands.
    pub fn bucket(&self) -> R2BucketGroup {
        R2BucketGroup
    }

    /// Access R2 object sub-commands.
    pub fn object(&self) -> R2ObjectGroup {
        R2ObjectGroup
    }
}

// ── R2 Bucket Group ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct R2BucketGroup;

// impl R2BucketGroup {
//     pub fn create(&self, name: impl Into<String>) -> R2BucketCreateBuilder { R2BucketCreateBuilder::new(name) }
//     pub fn list(&self) -> R2BucketListBuilder { R2BucketListBuilder::new() }
//     pub fn info(&self, name: impl Into<String>) -> R2BucketInfoBuilder { R2BucketInfoBuilder::new(name) }
//     pub fn delete(&self, name: impl Into<String>) -> R2BucketDeleteBuilder { R2BucketDeleteBuilder::new(name) }
// }

// ── R2 Object Group ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct R2ObjectGroup;

// impl R2ObjectGroup {
//     pub fn get(&self, key: impl Into<String>) -> R2ObjectGetBuilder { R2ObjectGetBuilder::new(key) }
//     pub fn put(&self, key: impl Into<String>) -> R2ObjectPutBuilder { R2ObjectPutBuilder::new(key) }
//     pub fn delete(&self, key: impl Into<String>) -> R2ObjectDeleteBuilder { R2ObjectDeleteBuilder::new(key) }
// }

// ── Pages Group ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PagesGroup;

impl PagesGroup {
    /// Access Pages project sub-commands.
    pub fn project(&self) -> PagesProjectGroup {
        PagesProjectGroup
    }

    // pub fn deploy(&self) -> PagesDeployBuilder { PagesDeployBuilder::new() }

    /// Access Pages deployment sub-commands.
    pub fn deployment(&self) -> PagesDeploymentGroup {
        PagesDeploymentGroup
    }
}

// ── Pages Project Group ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PagesProjectGroup;

// impl PagesProjectGroup {
//     pub fn list(&self) -> PagesProjectListBuilder { PagesProjectListBuilder::new() }
//     pub fn create(&self, name: impl Into<String>) -> PagesProjectCreateBuilder { PagesProjectCreateBuilder::new(name) }
//     pub fn delete(&self, name: impl Into<String>) -> PagesProjectDeleteBuilder { PagesProjectDeleteBuilder::new(name) }
// }

// ── Pages Deployment Group ──────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PagesDeploymentGroup;

// impl PagesDeploymentGroup {
//     pub fn list(&self) -> PagesDeploymentListBuilder { PagesDeploymentListBuilder::new() }
//     pub fn delete(&self, deployment_id: impl Into<String>) -> PagesDeploymentDeleteBuilder { PagesDeploymentDeleteBuilder::new(deployment_id) }
// }

// ── Queues Group ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct QueuesGroup;

impl QueuesGroup {
    /// Access Queues consumer sub-commands.
    pub fn consumer(&self) -> QueuesConsumerGroup {
        QueuesConsumerGroup
    }

    // pub fn list(&self) -> QueuesListBuilder { QueuesListBuilder::new() }
    // pub fn create(&self, name: impl Into<String>) -> QueuesCreateBuilder { QueuesCreateBuilder::new(name) }
    // pub fn update(&self, name: impl Into<String>) -> QueuesUpdateBuilder { QueuesUpdateBuilder::new(name) }
    // pub fn delete(&self, name: impl Into<String>) -> QueuesDeleteBuilder { QueuesDeleteBuilder::new(name) }
    // pub fn info(&self, name: impl Into<String>) -> QueuesInfoBuilder { QueuesInfoBuilder::new(name) }
    // pub fn pause_delivery(&self, name: impl Into<String>) -> QueuesPauseDeliveryBuilder { QueuesPauseDeliveryBuilder::new(name) }
    // pub fn resume_delivery(&self, name: impl Into<String>) -> QueuesResumeDeliveryBuilder { QueuesResumeDeliveryBuilder::new(name) }
    // pub fn purge(&self, name: impl Into<String>) -> QueuesPurgeBuilder { QueuesPurgeBuilder::new(name) }
}

// ── Queues Consumer Group ───────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct QueuesConsumerGroup;

// impl QueuesConsumerGroup {
//     pub fn add(&self, queue: impl Into<String>, script: impl Into<String>) -> QueuesConsumerAddBuilder { QueuesConsumerAddBuilder::new(queue, script) }
//     pub fn remove(&self, queue: impl Into<String>, script: impl Into<String>) -> QueuesConsumerRemoveBuilder { QueuesConsumerRemoveBuilder::new(queue, script) }
// }

// ── Vectorize Group ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct VectorizeGroup;

// impl VectorizeGroup {
//     pub fn list(&self) -> VectorizeListBuilder { VectorizeListBuilder::new() }
//     pub fn create(&self, name: impl Into<String>) -> VectorizeCreateBuilder { VectorizeCreateBuilder::new(name) }
//     pub fn delete(&self, name: impl Into<String>) -> VectorizeDeleteBuilder { VectorizeDeleteBuilder::new(name) }
//     pub fn get(&self, name: impl Into<String>) -> VectorizeGetBuilder { VectorizeGetBuilder::new(name) }
//     pub fn info(&self, name: impl Into<String>) -> VectorizeInfoBuilder { VectorizeInfoBuilder::new(name) }
//     pub fn insert(&self, name: impl Into<String>) -> VectorizeInsertBuilder { VectorizeInsertBuilder::new(name) }
//     pub fn upsert(&self, name: impl Into<String>) -> VectorizeUpsertBuilder { VectorizeUpsertBuilder::new(name) }
//     pub fn query(&self, name: impl Into<String>) -> VectorizeQueryBuilder { VectorizeQueryBuilder::new(name) }
//     pub fn get_vectors(&self, name: impl Into<String>) -> VectorizeGetVectorsBuilder { VectorizeGetVectorsBuilder::new(name) }
//     pub fn delete_vectors(&self, name: impl Into<String>) -> VectorizeDeleteVectorsBuilder { VectorizeDeleteVectorsBuilder::new(name) }
// }

// ── Hyperdrive Group ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct HyperdriveGroup;

// impl HyperdriveGroup {
//     pub fn list(&self) -> HyperdriveListBuilder { HyperdriveListBuilder::new() }
//     pub fn create(&self, name: impl Into<String>) -> HyperdriveCreateBuilder { HyperdriveCreateBuilder::new(name) }
//     pub fn delete(&self, id: impl Into<String>) -> HyperdriveDeleteBuilder { HyperdriveDeleteBuilder::new(id) }
//     pub fn get(&self, id: impl Into<String>) -> HyperdriveGetBuilder { HyperdriveGetBuilder::new(id) }
//     pub fn update(&self, id: impl Into<String>) -> HyperdriveUpdateBuilder { HyperdriveUpdateBuilder::new(id) }
// }

// ── Workflows Group ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct WorkflowsGroup;

impl WorkflowsGroup {
    /// Access Workflows instances sub-commands.
    pub fn instances(&self) -> WorkflowsInstancesGroup {
        WorkflowsInstancesGroup
    }

    // pub fn list(&self) -> WorkflowsListBuilder { WorkflowsListBuilder::new() }
    // pub fn describe(&self, name: impl Into<String>) -> WorkflowsDescribeBuilder { WorkflowsDescribeBuilder::new(name) }
    // pub fn delete(&self, name: impl Into<String>) -> WorkflowsDeleteBuilder { WorkflowsDeleteBuilder::new(name) }
    // pub fn trigger(&self, name: impl Into<String>) -> WorkflowsTriggerBuilder { WorkflowsTriggerBuilder::new(name) }
}

// ── Workflows Instances Group ───────────────────────────────────────

#[derive(Debug, Clone)]
pub struct WorkflowsInstancesGroup;

// impl WorkflowsInstancesGroup {
//     pub fn list(&self, name: impl Into<String>) -> WorkflowsInstancesListBuilder { WorkflowsInstancesListBuilder::new(name) }
//     pub fn describe(&self, name: impl Into<String>) -> WorkflowsInstancesDescribeBuilder { WorkflowsInstancesDescribeBuilder::new(name) }
//     pub fn terminate(&self, name: impl Into<String>, instance_id: impl Into<String>) -> WorkflowsInstancesTerminateBuilder { WorkflowsInstancesTerminateBuilder::new(name, instance_id) }
//     pub fn pause(&self, name: impl Into<String>, instance_id: impl Into<String>) -> WorkflowsInstancesPauseBuilder { WorkflowsInstancesPauseBuilder::new(name, instance_id) }
//     pub fn resume(&self, name: impl Into<String>, instance_id: impl Into<String>) -> WorkflowsInstancesResumeBuilder { WorkflowsInstancesResumeBuilder::new(name, instance_id) }
// }
