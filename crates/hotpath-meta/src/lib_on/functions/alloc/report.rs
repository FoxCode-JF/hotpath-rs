use crate::ProfilingMode;
use std::collections::HashMap;
use std::time::Duration;

use super::state::FunctionStats;
use crate::output::{MetricType, MetricsProvider};

pub struct StatsData<'a> {
    pub stats: &'a HashMap<u32, FunctionStats>,
    pub total_elapsed: Duration,
    pub percentiles: Vec<u8>,
    pub caller_name: &'static str,
    pub limit: usize,
}

pub struct TimingStatsData<'a> {
    pub stats: &'a HashMap<u32, FunctionStats>,
    pub total_elapsed: Duration,
    pub percentiles: Vec<u8>,
    pub caller_name: &'static str,
    pub limit: usize,
}

impl<'a> MetricsProvider<'a> for StatsData<'a> {
    fn new(
        stats: &'a HashMap<u32, FunctionStats>,
        total_elapsed: Duration,
        percentiles: Vec<u8>,
        caller_name: &'static str,
        limit: usize,
    ) -> Self {
        Self {
            stats,
            total_elapsed,
            percentiles,
            caller_name,
            limit,
        }
    }

    fn profiling_mode(&self) -> ProfilingMode {
        use crate::lib_on::functions::alloc::shared::{alloc_metric, AllocMetric};
        match alloc_metric() {
            AllocMetric::Bytes => ProfilingMode::AllocBytes,
            AllocMetric::Count => ProfilingMode::AllocCount,
        }
    }

    fn description(&self) -> String {
        use crate::lib_on::functions::alloc::shared::{alloc_metric, AllocMetric};
        let metric = match alloc_metric() {
            AllocMetric::Bytes => "bytes",
            AllocMetric::Count => "count",
        };
        if super::shared::is_alloc_self_enabled() {
            format!(
                "Exclusive allocation {} by each function (excluding nested calls).",
                metric
            )
        } else {
            format!(
                "Cumulative allocation {} during each function call (including nested calls).",
                metric
            )
        }
    }

    fn percentiles(&self) -> Vec<u8> {
        self.percentiles.clone()
    }

    fn function_ids(&self) -> HashMap<&'static str, u32> {
        self.stats
            .values()
            .map(|stat| (stat.name, stat.id))
            .collect()
    }

    fn metric_data(&self) -> Vec<(&'static str, Vec<MetricType>)> {
        use crate::lib_on::functions::alloc::shared::{alloc_metric, AllocMetric};

        let exclude_wrapper = *crate::functions::EXCLUDE_WRAPPER;
        let use_count = alloc_metric() == AllocMetric::Count;

        let primary_value = |s: &FunctionStats| -> u64 {
            if use_count {
                s.total_count()
            } else {
                s.total_bytes()
            }
        };

        let mut entries: Vec<_> = self
            .stats
            .values()
            .filter(|s| s.has_data && !(exclude_wrapper && s.wrapper))
            .collect();

        entries.sort_by(|a, b| {
            primary_value(b)
                .cmp(&primary_value(a))
                .then_with(|| a.name.cmp(b.name))
        });

        let entries = if self.limit > 0 {
            entries.into_iter().take(self.limit).collect::<Vec<_>>()
        } else {
            entries
        };

        let grand_total: u64 = if *crate::functions::EXCLUDE_WRAPPER {
            self.stats
                .values()
                .filter(|s| !s.wrapper && s.has_data)
                .map(|s| primary_value(s))
                .sum()
        } else if super::shared::is_alloc_self_enabled() {
            self.stats
                .values()
                .filter(|s| s.has_data)
                .map(|s| primary_value(s))
                .sum()
        } else {
            let wrapper_total = self
                .stats
                .values()
                .find(|s| s.wrapper && s.has_data)
                .map(|s| primary_value(s));

            wrapper_total.unwrap_or_else(|| {
                self.stats
                    .values()
                    .filter(|s| s.has_data)
                    .map(|s| primary_value(s))
                    .sum()
            })
        };

        entries
            .into_iter()
            .map(|stats| {
                let percentage = if grand_total > 0 {
                    (primary_value(stats) as f64 / grand_total as f64) * 100.0
                } else {
                    0.0
                };

                let mut metrics = if stats.is_async {
                    vec![MetricType::CallsCount(stats.count), MetricType::Unsupported]
                } else {
                    vec![
                        MetricType::CallsCount(stats.count),
                        MetricType::Alloc(stats.avg_bytes(), stats.avg_count()),
                    ]
                };

                for &p in &self.percentiles {
                    if stats.is_async {
                        metrics.push(MetricType::Unsupported);
                    } else {
                        let bytes_total = stats.bytes_total_percentile(p as f64);
                        let count_total = stats.count_total_percentile(p as f64);
                        metrics.push(MetricType::Alloc(bytes_total, count_total));
                    }
                }

                if stats.is_async {
                    metrics.push(MetricType::Unsupported);
                    metrics.push(MetricType::Unsupported);
                } else {
                    metrics.push(MetricType::Alloc(stats.total_bytes(), stats.total_count()));
                    metrics.push(MetricType::Percentage((percentage * 100.0) as u64));
                }

                (stats.name, metrics)
            })
            .collect()
    }

    fn total_elapsed(&self) -> u64 {
        self.total_elapsed.as_nanos() as u64
    }

    fn caller_name(&self) -> &str {
        self.caller_name
    }

    fn entry_counts(&self) -> (usize, usize) {
        let exclude_wrapper = *crate::functions::EXCLUDE_WRAPPER;
        let total_count = self
            .stats
            .values()
            .filter(|s| s.has_data && !(exclude_wrapper && s.wrapper))
            .count();

        let displayed_count = if self.limit > 0 && self.limit < total_count {
            self.limit
        } else {
            total_count
        };

        (displayed_count, total_count)
    }
}

impl<'a> MetricsProvider<'a> for TimingStatsData<'a> {
    fn new(
        stats: &'a HashMap<u32, FunctionStats>,
        total_elapsed: Duration,
        percentiles: Vec<u8>,
        caller_name: &'static str,
        limit: usize,
    ) -> Self {
        Self {
            stats,
            total_elapsed,
            percentiles,
            caller_name,
            limit,
        }
    }

    fn profiling_mode(&self) -> ProfilingMode {
        ProfilingMode::Timing
    }

    fn description(&self) -> String {
        "Function execution time metrics.".to_string()
    }

    fn percentiles(&self) -> Vec<u8> {
        self.percentiles.clone()
    }

    fn function_ids(&self) -> HashMap<&'static str, u32> {
        self.stats
            .values()
            .map(|stat| (stat.name, stat.id))
            .collect()
    }

    fn metric_data(&self) -> Vec<(&'static str, Vec<MetricType>)> {
        let exclude_wrapper = *crate::functions::EXCLUDE_WRAPPER;
        let mut entries: Vec<_> = self
            .stats
            .values()
            .filter(|s| s.has_data && !(exclude_wrapper && s.wrapper))
            .collect();

        entries.sort_by(|a, b| {
            b.total_duration_ns
                .cmp(&a.total_duration_ns)
                .then_with(|| a.name.cmp(b.name))
        });

        let entries = if self.limit > 0 {
            entries.into_iter().take(self.limit).collect::<Vec<_>>()
        } else {
            entries
        };

        let reference_total = if exclude_wrapper {
            self.stats
                .values()
                .filter(|s| !s.wrapper && s.has_data)
                .map(|s| s.total_duration_ns)
                .sum::<u64>()
        } else {
            let wrapper_total = self
                .stats
                .values()
                .find(|s| s.wrapper)
                .map(|s| s.total_duration_ns);
            wrapper_total.unwrap_or(self.total_elapsed.as_nanos() as u64)
        };

        entries
            .into_iter()
            .map(|stats| {
                let percentage = if reference_total > 0 {
                    (stats.total_duration_ns as f64 / reference_total as f64) * 100.0
                } else {
                    0.0
                };

                let mut metrics = vec![
                    MetricType::CallsCount(stats.count),
                    MetricType::DurationNs(stats.avg_duration_ns()),
                ];

                for &p in &self.percentiles {
                    let duration_ns = stats.duration_percentile(p as f64);
                    metrics.push(MetricType::DurationNs(duration_ns));
                }

                metrics.push(MetricType::DurationNs(stats.total_duration_ns));
                metrics.push(MetricType::Percentage((percentage * 100.0) as u64));

                (stats.name, metrics)
            })
            .collect()
    }

    fn total_elapsed(&self) -> u64 {
        self.total_elapsed.as_nanos() as u64
    }

    fn caller_name(&self) -> &str {
        self.caller_name
    }

    fn entry_counts(&self) -> (usize, usize) {
        let exclude_wrapper = *crate::functions::EXCLUDE_WRAPPER;
        let total_count = self
            .stats
            .values()
            .filter(|s| s.has_data && !(exclude_wrapper && s.wrapper))
            .count();

        let displayed_count = if self.limit > 0 && self.limit < total_count {
            self.limit
        } else {
            total_count
        };

        (displayed_count, total_count)
    }
}
