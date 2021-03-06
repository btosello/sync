use std::collections::HashMap;

use convert_case::{Case, Casing};
use futures::lock::Mutex;
use indy_api_types::errors::{IndyErrorKind, IndyResult, IndyResultExt};
use serde_json::{Map, Value};

use models::{CommandCounters, MetricsValue};

use crate::services::metrics::command_metrics::CommandMetric;

pub mod command_metrics;
pub mod models;

const COMMANDS_COUNT: usize = MetricsService::commands_count();

pub struct MetricsService {
    queued_counters: Mutex<CommandCounters>,
    executed_counters: Mutex<[CommandCounters; COMMANDS_COUNT]>,
    callback_counters: Mutex<[CommandCounters; COMMANDS_COUNT]>,
}

impl MetricsService {
    pub fn new() -> Self {
        MetricsService {
            queued_counters: Mutex::new(CommandCounters::new()),
            executed_counters: Mutex::new([CommandCounters::new(); COMMANDS_COUNT]),
            callback_counters: Mutex::new([CommandCounters::new(); COMMANDS_COUNT]),
        }
    }

    pub async fn cmd_left_queue(&self, _command_metric: CommandMetric, duration: u128) {
        self.queued_counters.lock().await.add(duration);
    }

    pub async fn cmd_executed(&self, command_metric: CommandMetric, duration: u128) {
        self.executed_counters.lock().await[command_metric as usize].add(duration);
    }

    pub async fn cmd_callback(&self, command_metric: CommandMetric, duration: u128) {
        self.callback_counters.lock().await[command_metric as usize].add(duration);
    }

    pub fn cmd_name(index: usize) -> String {
        CommandMetric::from(index).to_string().to_case(Case::Snake)
    }

    const fn commands_count() -> usize {
        CommandMetric::VARIANT_COUNT
    }

    pub fn get_command_tags(
        command: String,
        stage: String,
    ) -> HashMap<String, String> {
        let mut tags = HashMap::<String, String>::new();
        tags.insert("command".to_owned(), command.to_owned());
        tags.insert("stage".to_owned(), stage.to_owned());
        tags
    }

    pub async fn append_command_metrics(&self, metrics_map: &mut Map<String, Value>) -> IndyResult<()> {
        let mut commands_count = Vec::new();
        let mut commands_duration_ms = Vec::new();
        let mut commands_duration_ms_bucket = Vec::new();

        for index in 0..MetricsService::commands_count() {
            let command_name = MetricsService::cmd_name(index);
            let tags_executed = MetricsService::get_command_tags(
                command_name.to_owned(),
                "executed".to_owned(),
            );
            let tags_cb = MetricsService::get_command_tags(
                command_name.to_owned(),
                "callback".to_owned(),
            );

            let exec_counters = self.executed_counters.lock().await;
            commands_count.push(Self::get_metric_json(exec_counters[index].count as usize, tags_executed.clone())?);
            commands_duration_ms.push(Self::get_metric_json(exec_counters[index].duration_ms_sum as usize, tags_executed.clone())?);

            let cb_counters = self.callback_counters.lock().await;
            commands_count.push(Self::get_metric_json(cb_counters[index].count as usize, tags_cb.clone())?);
            commands_duration_ms.push(Self::get_metric_json(cb_counters[index].duration_ms_sum as usize, tags_cb.clone())?);

            for (executed_bucket, le) in exec_counters[index].duration_ms_bucket.iter().zip(models::LIST_LE.iter()) {
                let mut tags = tags_executed.clone();
                tags.insert("le".to_owned(), le.to_string());
                commands_duration_ms_bucket.push(Self::get_metric_json(*executed_bucket as usize, tags)?);
            }

            for (cb_bucket, le) in cb_counters[index].duration_ms_bucket.iter().zip(models::LIST_LE.iter()) {
                let mut tags = tags_cb.clone();
                tags.insert("le".to_owned(), le.to_string());
                commands_duration_ms_bucket.push(Self::get_metric_json(*cb_bucket as usize, tags)?);
            }
        }

        let tags_queued = {
            let mut tags = HashMap::<String, String>::new();
            tags.insert("stage".to_owned(), "queued".to_owned());
            tags
        };
        let queued_counters = self.queued_counters.lock().await;
        commands_duration_ms.push(Self::get_metric_json(queued_counters.duration_ms_sum as usize, tags_queued.clone())?);
        commands_count.push(Self::get_metric_json(queued_counters.count as usize, tags_queued.clone())?);

        for (queued_bucket, le) in queued_counters.duration_ms_bucket.iter().rev().zip(models::LIST_LE.iter().rev()) {
            let mut tags = tags_queued.clone();
            tags.insert("le".to_owned(), le.to_string());
            commands_duration_ms_bucket.push(Self::get_metric_json(*queued_bucket as usize, tags)?);
        }

        metrics_map.insert(
            "command_duration_ms_count".to_owned(),
            serde_json::to_value(commands_count)
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );
        metrics_map.insert(
            "command_duration_ms_sum".to_owned(),
            serde_json::to_value(commands_duration_ms)
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );
        metrics_map.insert(
            "command_duration_ms_bucket".to_owned(),
            serde_json::to_value(commands_duration_ms_bucket)
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );

        Ok(())
    }

    pub(crate) fn get_metric_json(value: usize, tags: HashMap<String, String>) -> IndyResult<Value> {
        let res = serde_json::to_value(MetricsValue::new(
            value,
            tags,
        )).to_indy(IndyErrorKind::IOError, "Unable to convert json")?;

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[async_std::test]
    async fn test_counters_are_initialized() {
        let metrics_service = MetricsService::new();
        assert_eq!(metrics_service.executed_counters.lock().await.len(), COMMANDS_COUNT);
    }

    #[async_std::test]
    async fn test_cmd_left_queue_increments_relevant_queued_counters() {
        let metrics_service = MetricsService::new();
        let index = CommandMetric::IssuerCommandCreateSchema;
        let duration1 = 5u128;
        let duration2 = 2u128;

        metrics_service.cmd_left_queue(index, duration1).await;

        {
            let queued_counters = metrics_service.queued_counters.lock().await;
            assert_eq!(queued_counters.count, 1);
            assert_eq!(queued_counters.duration_ms_sum, duration1);
            assert_eq!(queued_counters
                           .duration_ms_bucket[
                           queued_counters
                               .duration_ms_bucket.len() - 1
                           ],
                       1
            );
        }

        metrics_service.cmd_left_queue(index, duration2).await;

        {
            let queued_counters = metrics_service.queued_counters.lock().await;
            assert_eq!(queued_counters.count, 1 + 1);
            assert_eq!(queued_counters.duration_ms_sum,
                       duration1 + duration2);
            assert_eq!(queued_counters
                           .duration_ms_bucket[
                           queued_counters
                               .duration_ms_bucket.len() - 1
                           ],
                       2
            );
        }

        let executed_counters = metrics_service.executed_counters.lock().await;
        assert_eq!(executed_counters[index as usize].count, 0);
        assert_eq!(executed_counters[index as usize].duration_ms_sum, 0);
        assert_eq!(executed_counters[index as usize]
                       .duration_ms_bucket[
                            executed_counters[index as usize]
                            .duration_ms_bucket.len()-1
                       ],
                   0
        );
    }

    #[async_std::test]
    async fn test_cmd_executed_increments_relevant_executed_counters() {
        let metrics_service = MetricsService::new();
        let index = CommandMetric::IssuerCommandCreateSchema;
        let duration1 = 5u128;
        let duration2 = 2u128;

        metrics_service.cmd_executed(index, duration1).await;

        assert_eq!(metrics_service.executed_counters.lock().await[index as usize].count, 1);
        assert_eq!(metrics_service.executed_counters.lock().await[index as usize].duration_ms_sum, duration1);

        metrics_service.cmd_executed(index, duration2).await;

        assert_eq!(metrics_service.queued_counters.lock().await.count, 0);
        assert_eq!(metrics_service.queued_counters.lock().await.duration_ms_sum, 0);
        assert_eq!(metrics_service.executed_counters.lock().await[index as usize].count, 1 + 1);
        assert_eq!(metrics_service.executed_counters.lock().await[index as usize].duration_ms_sum, duration1 + duration2);
    }

    #[async_std::test]
    async fn test_append_command_metrics() {
        let metrics_service = MetricsService::new();
        let mut metrics_map = serde_json::Map::new();

        metrics_service.append_command_metrics(&mut metrics_map).await.unwrap();

        assert!(metrics_map.contains_key("command_duration_ms_count"));
        assert!(metrics_map.contains_key("command_duration_ms_sum"));
        assert!(metrics_map.contains_key("command_duration_ms_bucket"));

        assert_eq!(
            metrics_map
                .get("command_duration_ms_count")
                .unwrap()
                .as_array()
                .unwrap()
                .len(),
            COMMANDS_COUNT * 2 + 1
        );
        assert_eq!(
            metrics_map
                .get("command_duration_ms_sum")
                .unwrap()
                .as_array()
                .unwrap()
                .len(),
            COMMANDS_COUNT * 2 + 1
        );
        assert_eq!(
            metrics_map
                .get("command_duration_ms_bucket")
                .unwrap()
                .as_array()
                .unwrap()
                .len(),
            COMMANDS_COUNT * 16 * 2 /* cb and executed buckets */ + 16 /* queued buckets */
        );

        let commands_count = metrics_map
            .get("command_duration_ms_count")
            .unwrap()
            .as_array()
            .unwrap();
        let commands_duration_ms = metrics_map
            .get("command_duration_ms_sum")
            .unwrap()
            .as_array()
            .unwrap();
        let commands_duration_ms_bucket = metrics_map
            .get("command_duration_ms_bucket")
            .unwrap()
            .as_array()
            .unwrap();

        let expected_commands_count = [
            generate_json("payments_command_build_set_txn_fees_req_ack", "executed", 0),
            generate_json("cache_command_purge_cred_def_cache", "executed", 0),
            json!({"tags": {"stage": "queued"}, "value": 0})
        ];

        let expected_commands_duration_ms = [
            generate_json("payments_command_build_set_txn_fees_req_ack", "executed", 0),
            generate_json("cache_command_purge_cred_def_cache", "executed", 0),
            json!({"tags": {"stage": "queued"}, "value": 0})
        ];

        let expected_commands_duration_ms_bucket = [
            json!({"tags": {"command": "payments_command_build_set_txn_fees_req_ack", "stage": "executed", "le": "+Inf"}, "value": 0}),
            json!({"tags": {"command": "cache_command_purge_cred_def_cache", "stage": "executed", "le": "+Inf"}, "value": 0}),
            json!({"tags": {"stage": "queued", "le": "+Inf"}, "value": 0})
        ];

        for command in &expected_commands_count {
            assert!(commands_count.contains(&command));
        }

        for command in &expected_commands_duration_ms {
            assert!(commands_duration_ms.contains(&command));
        }

        for command in &expected_commands_duration_ms_bucket {
            assert!(commands_duration_ms_bucket.contains(&command));
        }
    }

    fn generate_json(command: &str, stage: &str, value: usize) -> Value {
        json!({"tags":{"command": command, "stage": stage} ,"value": value})
    }
}