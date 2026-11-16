//! Opt-in checks against a real AOSC OS amo service. Normal test runs never change packages.

use aoska::common::{amo::AmoBackend, amo_types::PmOperationStart};
use std::time::Duration;

async fn completed(backend: &AmoBackend, operation: PmOperationStart) {
    let status = tokio::time::timeout(Duration::from_secs(600), async {
        loop {
            let status = backend.status(&operation.id).unwrap();
            if status.state != "running" {
                break status;
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    })
    .await
    .expect("timed out waiting for the real amo ResultReport");
    assert_eq!(status.state, "succeeded", "{:?}", status.message);
    assert!(!backend.logs(&operation.id).unwrap().lines.is_empty());
    println!("{} {}: {}", operation.operation, operation.id, status.state);
}

#[tokio::test]
#[ignore = "requires real amo/Polkit authorization; refreshes metadata and optionally installs/removes AOSKA_TEST_PACKAGE"]
async fn real_amo_queries_refresh_and_package_cycle() {
    let backend = AmoBackend::default();
    let capabilities = backend.capabilities().await.unwrap();
    assert_eq!(capabilities.raw["backend"], "amo");
    assert!(
        backend
            .package_state(&["git".into()])
            .await
            .unwrap()
            .packages[0]
            .installed
    );
    let missing = backend
        .package_state(&["aoska-this-package-does-not-exist-20261117".into()])
        .await
        .unwrap();
    assert!(!missing.packages[0].aoska_manageable);
    let summary = backend.update_summary().await.unwrap();
    assert_eq!(
        summary.total,
        backend.list_updates().await.unwrap().packages.len()
    );
    println!(
        "real amo: {} updates, {:?} security updates",
        summary.total, summary.security
    );

    completed(&backend, backend.start("refresh", &[]).await.unwrap()).await;

    let Ok(package) = std::env::var("AOSKA_TEST_PACKAGE") else {
        return;
    };
    let packages = vec![package.clone()];
    let before = backend.package_state(&packages).await.unwrap();
    assert!(before.packages[0].aoska_manageable);
    assert!(
        !before.packages[0].installed,
        "test package must initially be absent"
    );
    let plan = backend.plan("install", &packages).await.unwrap();
    assert!(
        plan.remove.is_empty(),
        "test must not remove existing software"
    );
    assert_eq!(
        plan.install.len(),
        1,
        "test must not install or upgrade dependencies"
    );
    assert_eq!(plan.install[0].name, package);
    completed(&backend, backend.start("install", &packages).await.unwrap()).await;
    assert!(backend.package_state(&packages).await.unwrap().packages[0].installed);

    let removal = backend.plan("remove", &packages).await.unwrap();
    assert!(removal.install.is_empty());
    assert_eq!(
        removal.remove.len(),
        1,
        "test must only remove the package it installed"
    );
    assert_eq!(removal.remove[0].name, package);
    completed(&backend, backend.start("remove", &packages).await.unwrap()).await;
    assert!(!backend.package_state(&packages).await.unwrap().packages[0].installed);
    println!("test package {package} restored to its original absent state");
}
