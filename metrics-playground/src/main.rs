fn main() {
    // By default `prometheus::default_registry()` is used.
    let recorder = metrics_prometheus::install();

    // Either use `metrics` crate interfaces.
    metrics::increment_counter!("count", "whose" => "mine", "kind" => "owned");
    metrics::increment_counter!("count", "whose" => "mine", "kind" => "ref");
    metrics::increment_counter!("count", "kind" => "owned", "whose" => "dummy");

    // Or construct and provide `prometheus` metrics directly.
    recorder.register_metric(prometheus::Gauge::new("value", "help")?);

    let report = prometheus::TextEncoder::new()
        .encode_to_string(&prometheus::default_registry().gather())?;
    assert_eq!(
        report.trim(),
        r#"
# HELP count count
# TYPE count counter
count{kind="owned",whose="dummy"} 1
count{kind="owned",whose="mine"} 1
count{kind="ref",whose="mine"} 1
# HELP value help
# TYPE value gauge
value 0
    "#
        .trim(),
    );

    // Metrics can be described anytime after being registered in
    // `prometheus::Registry`.
    metrics::describe_counter!("count", "Example of counter.");
    metrics::describe_gauge!("value", "Example of gauge.");

    let report = prometheus::TextEncoder::new().encode_to_string(&recorder.registry().gather())?;
    assert_eq!(
        report.trim(),
        r#"
# HELP count Example of counter.
# TYPE count counter
count{kind="owned",whose="dummy"} 1
count{kind="owned",whose="mine"} 1
count{kind="ref",whose="mine"} 1
# HELP value Example of gauge.
# TYPE value gauge
value 0
    "#
        .trim(),
    );

    // Description can be changed multiple times and anytime.
    metrics::describe_counter!("count", "Another description.");

    // Even before a metric is registered in `prometheus::Registry`.
    metrics::describe_counter!("another", "Yet another counter.");
    metrics::increment_counter!("another");

    let report = prometheus::TextEncoder::new().encode_to_string(&recorder.registry().gather())?;
    assert_eq!(
        report.trim(),
        r#"
# HELP another Yet another counter.
# TYPE another counter
another 1
# HELP count Another description.
# TYPE count counter
count{kind="owned",whose="dummy"} 1
count{kind="owned",whose="mine"} 1
count{kind="ref",whose="mine"} 1
# HELP value Example of gauge.
# TYPE value gauge
value 0
    "#
        .trim(),
    );
}
