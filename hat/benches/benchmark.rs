use std::collections::HashMap;

use chrono::Local;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use serde_json::Map;
use tracing::info;

use hat::runtime::{
    device::{Device, DeviceType},
    event::{Event, EventType},
    HatRuntime,
};

pub fn benchmark_runtime_throughput(c: &mut Criterion) {
    info!("Creating tokio runtime...");

    let tokio_rt = tokio::runtime::Runtime::new().unwrap();

    info!("Creating hat runtime...");

    let hat_rt = tokio_rt.block_on(async {
        let hat_rt = HatRuntime::new().await;
        hat_rt
            .parse("bench.hat".into(), &include_str!("../src/test/bench.hat"))
            .await
            .unwrap();
        hat_rt
    });

    info!("Starting throughput benchmark...");

    let mut group = c.benchmark_group("hat_runtime");

    // each iteration = one “Element”
    group.throughput(Throughput::Elements(1));

    // 3) benchmark the async function itself
    group.bench_function("handle_event", move |b| {
        // `to_async` lets us .await inside the iteration

        b.to_async(&tokio_rt).iter({
            || {
                let hat_rt_clone = hat_rt.clone();
                async move {
                    hat_rt_clone
                        .dispatch_event(Event {
                            typ: EventType::Dummy,
                            datetime: Local::now(),
                            device: Device {
                                integration: "benchmark".into(),
                                id: "benchmark".into(),
                                name: None,
                                typ: DeviceType::Dummy,
                                state: None,
                                attributes: Map::new(),
                            },
                            parameters: HashMap::new(),
                        })
                        .await
                        .unwrap();
                }
            }
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_runtime_throughput);

criterion_main!(benches);
