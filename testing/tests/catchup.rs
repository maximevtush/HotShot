#[cfg(test)]
#[cfg_attr(
    feature = "tokio-executor",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(feature = "async-std-executor", async_std::test)]
async fn test_catchup() {
    use std::time::Duration;

    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        node_types::{SequencingMemoryImpl, SequencingTestTypes},
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::{TestMetadata, TimingData},
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let timing_data = TimingData {
        next_view_timeout: 1000,
        ..Default::default()
    };
    let mut metadata = TestMetadata::default();
    let catchup_nodes = vec![ChangeNode {
        idx: 2,
        updown: UpDown::Up,
    }];

    metadata.timing_data = timing_data;

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(Duration::new(1, 0), catchup_nodes)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_millis(10000),
            },
        );
    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        check_leaf: true,
        ..Default::default()
    };

    metadata
        .gen_launcher::<SequencingTestTypes, SequencingMemoryImpl>()
        .launch()
        .run_test()
        .await;
}
