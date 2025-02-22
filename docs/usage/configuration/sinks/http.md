---
description: Batches `log` events to a generic HTTP endpoint.
---

<!--
     THIS FILE IS AUTOOGENERATED!

     To make changes please edit the template located at:

     scripts/generate/templates/docs/usage/configuration/sinks/http.md.erb
-->

# http sink

![][images.http_sink]


The `http` sink batches [`log`][docs.log_event] events to a generic HTTP endpoint.

## Config File

{% code-tabs %}
{% code-tabs-item title="vector.toml (example)" %}
```coffeescript
[sinks.my_http_sink_id]
  # REQUIRED - General
  type = "http" # must be: "http"
  inputs = ["my-source-id"]
  encoding = "ndjson" # enum: "ndjson", "text"
  uri = "https://10.22.212.22:9000/endpoint"

  # OPTIONAL - General
  compression = "gzip" # no default, must be: "gzip"
  healthcheck_uri = "https://10.22.212.22:9000/_health" # no default

  # OPTIONAL - Batching
  batch_size = 1049000 # default, bytes
  batch_timeout = 5 # default, bytes

  # OPTIONAL - Requests
  rate_limit_duration = 1 # default, seconds
  rate_limit_num = 10 # default
  request_in_flight_limit = 10 # default
  request_timeout_secs = 30 # default, seconds
  retry_attempts = 10 # default
  retry_backoff_secs = 10 # default, seconds

  # OPTIONAL - Basic auth
  [sinks.my_http_sink_id.basic_auth]
    password = "password" # no default
    user = "username" # no default

  # OPTIONAL - Buffer
  [sinks.my_http_sink_id.buffer]
    type = "memory" # default, enum: "memory", "disk"
    when_full = "block" # default, enum: "block", "drop_newest"
    max_size = 104900000 # no default
    num_items = 500 # default

  # OPTIONAL - Headers
  [sinks.my_http_sink_id.headers]
    * = {name = "X-Powered-By", value = "Vector"} # no default
```
{% endcode-tabs-item %}
{% code-tabs-item title="vector.toml (schema)" %}
```coffeescript
[sinks.<sink-id>]
  # REQUIRED - General
  type = "http"
  inputs = ["<string>", ...]
  encoding = {"ndjson" | "text"}
  uri = "<string>"

  # OPTIONAL - General
  compression = "gzip"
  healthcheck_uri = "<string>"

  # OPTIONAL - Batching
  batch_size = <int>
  batch_timeout = <int>

  # OPTIONAL - Requests
  rate_limit_duration = <int>
  rate_limit_num = <int>
  request_in_flight_limit = <int>
  request_timeout_secs = <int>
  retry_attempts = <int>
  retry_backoff_secs = <int>

  # OPTIONAL - Basic auth
  [sinks.<sink-id>.basic_auth]
    password = "<string>"
    user = "<string>"

  # OPTIONAL - Buffer
  [sinks.<sink-id>.buffer]
    type = {"memory" | "disk"}
    when_full = {"block" | "drop_newest"}
    max_size = <int>
    num_items = <int>

  # OPTIONAL - Headers
  [sinks.<sink-id>.headers]
    * = "<string>"
```
{% endcode-tabs-item %}
{% code-tabs-item title="vector.toml (specification)" %}
```coffeescript
[sinks.http]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "http"
  type = "http"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The encoding format used to serialize the events before flushing.
  # 
  # * required
  # * no default
  # * enum: "ndjson", "text"
  encoding = "ndjson"
  encoding = "text"

  # The full URI to make HTTP requests to. This should include the protocol and
  # host, but can also include the port, path, and any other valid part of a URI.
  # 
  # * required
  # * no default
  uri = "https://10.22.212.22:9000/endpoint"

  # The compression strategy used to compress the payload before sending.
  # 
  # * optional
  # * no default
  # * must be: "gzip"
  compression = "gzip"

  # A URI that Vector can request in order to determine the service health.
  # 
  # * optional
  # * no default
  healthcheck_uri = "https://10.22.212.22:9000/_health"

  #
  # Batching
  #

  # The maximum size of a batch before it is flushed.
  # 
  # * optional
  # * default: 1049000
  # * unit: bytes
  batch_size = 1049000

  # The maximum age of a batch before it is flushed.
  # 
  # * optional
  # * default: 5
  # * unit: bytes
  batch_timeout = 5

  #
  # Requests
  #

  # The window used for the `request_rate_limit_num` option
  # 
  # * optional
  # * default: 1
  # * unit: seconds
  rate_limit_duration = 1

  # The maximum number of requests allowed within the `rate_limit_duration`
  # window.
  # 
  # * optional
  # * default: 10
  rate_limit_num = 10

  # The maximum number of in-flight requests allowed at any given time.
  # 
  # * optional
  # * default: 10
  request_in_flight_limit = 10

  # The maximum time a request can take before being aborted.
  # 
  # * optional
  # * default: 30
  # * unit: seconds
  request_timeout_secs = 30

  # The maximum number of retries to make for failed requests.
  # 
  # * optional
  # * default: 10
  retry_attempts = 10

  # The amount of time to wait before attempting a failed request again.
  # 
  # * optional
  # * default: 10
  # * unit: seconds
  retry_backoff_secs = 10

  #
  # Basic auth
  #

  [sinks.http.basic_auth]
    # The basic authentication password.
    # 
    # * optional
    # * no default
    password = "password"

    # The basic authentication user name.
    # 
    # * optional
    # * no default
    user = "username"

  #
  # Buffer
  #

  [sinks.http.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # Only relevant when `type` is `disk`. The maximum size of the buffer on the
    # disk.
    # 
    # * optional
    # * no default
    max_size = 104900000

    # Only relevant when `type` is `memory`. The maximum number of events allowed
    # in the buffer.
    # 
    # * optional
    # * default: 500
    num_items = 500

  #
  # Headers
  #

  [sinks.http.headers]
    # A custom header to be added to each outgoing HTTP request.
    # 
    # * optional
    # * no default
    X-Powered-By = "Vector"
```
{% endcode-tabs-item %}
{% endcode-tabs %}

## Options

| Key  | Type  | Description |
|:-----|:-----:|:------------|
| **REQUIRED** - General | | |
| `type` | `string` | The component type<br />`required` `enum: "http"` |
| `inputs` | `[string]` | A list of upstream [source][docs.sources] or [transform][docs.transforms] IDs. See [Config Composition][docs.config_composition] for more info.<br />`required` `example: ["my-source-id"]` |
| `encoding` | `string` | The encoding format used to serialize the events before flushing.<br />`required` `enum: "ndjson", "text"` |
| `uri` | `string` | The full URI to make HTTP requests to. This should include the protocol and host, but can also include the port, path, and any other valid part of a URI.<br />`required` `example: (see above)` |
| **OPTIONAL** - General | | |
| `compression` | `string` | The compression strategy used to compress the payload before sending.<br />`no default` `enum: "gzip"` |
| `healthcheck_uri` | `string` | A URI that Vector can request in order to determine the service health.<br />`no default` `example: (see above)` |
| **OPTIONAL** - Batching | | |
| `batch_size` | `int` | The maximum size of a batch before it is flushed.<br />`default: 1049000` `unit: bytes` |
| `batch_timeout` | `int` | The maximum age of a batch before it is flushed.<br />`default: 5` `unit: bytes` |
| **OPTIONAL** - Requests | | |
| `rate_limit_duration` | `int` | The window used for the `request_rate_limit_num` option<br />`default: 1` `unit: seconds` |
| `rate_limit_num` | `int` | The maximum number of requests allowed within the `rate_limit_duration` window.<br />`default: 10` |
| `request_in_flight_limit` | `int` | The maximum number of in-flight requests allowed at any given time.<br />`default: 10` |
| `request_timeout_secs` | `int` | The maximum time a request can take before being aborted.<br />`default: 30` `unit: seconds` |
| `retry_attempts` | `int` | The maximum number of retries to make for failed requests.<br />`default: 10` |
| `retry_backoff_secs` | `int` | The amount of time to wait before attempting a failed request again.<br />`default: 10` `unit: seconds` |
| **OPTIONAL** - Basic auth | | |
| `password` | `string` | The basic authentication password.<br />`no default` `example: "password"` |
| `user` | `string` | The basic authentication user name.<br />`no default` `example: "username"` |
| **OPTIONAL** - Buffer | | |
| `type` | `string` | The buffer's type / location. `disk` buffers are persistent and will be retained between restarts.<br />`default: "memory"` `enum: "memory", "disk"` |
| `when_full` | `string` | The behavior when the buffer becomes full.<br />`default: "block"` `enum: "block", "drop_newest"` |
| `max_size` | `int` | Only relevant when `type` is `disk`. The maximum size of the buffer on the disk.<br />`no default` `example: 104900000` |
| `num_items` | `int` | Only relevant when `type` is `memory`. The maximum number of [events][docs.event] allowed in the buffer.<br />`default: 500` |
| **OPTIONAL** - Headers | | |
| `*` | `string` | A custom header to be added to each outgoing HTTP request.<br />`no default` `example: (see above)` |

## Examples

The `http` sink batches [`log`][docs.log_event] up to the `batch_size` or
`batch_timeout` options. When flushed, Vector will write to a generic HTTP
endpoint. The encoding is dictated by the `encoding` option. For example:

```http
POST <host>/_bulk HTTP/1.1
Host: <host>
Content-Type: application/x-ndjson
Content-Length: 654

{ "index" : { "_index" : "<index>" } }
{"timestamp": 1557932537, "message": "GET /roi/evolve/embrace/transparent", "host": "Stracke8362", "process_id": 914, "remote_addr": "30.163.82.140", "response_code": 504, "bytes": 29763} 
{ "index" : { "_index" : "<index>" } }
{"timestamp": 1557933548, "message": "PUT /value-added/b2b", "host": "Wiza2458", "process_id": 775, "remote_addr": "30.163.82.140", "response_code": 503, "bytes": 9468}
{ "index" : { "_index" : "<index>" } }
{"timestamp": 1557933742, "message": "DELETE /reinvent/interfaces", "host": "Herman3087", "process_id": 775, "remote_addr": "43.246.221.247", "response_code": 503, "bytes": 9700}
```

## How It Works

### Buffering, Batching, & Partitioning

![][images.sink-flow-partitioned]

The `http` sink buffers, batches, and
partitions data as shown in the diagram above. You'll notice that Vector treats
these concepts differently, instead of treating them as global concepts, Vector
treats them as sink specific concepts. This isolates sinks, ensuring services
disruptions are contained and [delivery guarantees][docs.guarantees] are
honored.

#### Buffers types

The `buffer.type` option allows you to control buffer resource usage:

| Type     | Description                                                                                                    |
|:---------|:---------------------------------------------------------------------------------------------------------------|
| `memory` | Pros: Fast. Cons: Not persisted across restarts. Possible data loss in the event of a crash. Uses more memory. |
| `disk`   | Pros: Persisted across restarts, durable. Uses much less memory. Cons: Slower, see below.                      |

#### Buffer overflow

The `buffer.when_full` option allows you to control the behavior when the
buffer overflows:

| Type          | Description                                                                                                                        |
|:--------------|:-----------------------------------------------------------------------------------------------------------------------------------|
| `block`       | Applies back pressure until the buffer makes room. This will help to prevent data loss but will cause data to pile up on the edge. |
| `drop_newest` | Drops new data as it's received. This data is lost. This should be used when performance is the highest priority.                  |

#### Batch flushing

Batches are flushed when 1 of 2 conditions are met:

1. The batch age meets or exceeds the configured `batch_timeout` (default: `5 bytes`).
2. The batch size meets or exceeds the configured `batch_size` (default: `1049000 bytes`).

#### Partitioning

Partitioning is controlled via the 
options and allows you to dynamically partition data. You'll notice that
[`strftime` specifiers][url.strftime_specifiers] are allowed in the values,
enabling dynamic partitioning. The interpolated result is effectively the
internal batch partition key. Let's look at a few examples:

| Value | Interpolation | Desc |
|:------|:--------------|:-----|
| `date=%F` | `date=2019-05-02` | Partitions data by the event's day. |
| `date=%Y` | `date=2019` | Partitions data by the event's year. |
| `timestamp=%s` | `timestamp=1562450045` | Partitions data by the unix timestamp. |

### Compression

The `http` sink compresses payloads before
flushing. This helps to reduce the payload size, ultimately reducing bandwidth
and cost. This is controlled via the `compression` option. Each compression
type is described in more detail below:

| Compression | Description |
| :---------- | :---------- |

| `gzip` |  |


### Delivery Guarantee

This component offers an [**at least once** delivery guarantee][docs.at_least_once_delivery]
if your [pipeline is configured to achieve this][docs.at_least_once_delivery].

### Encodings

The `http` sink encodes events before writing
them downstream. This is controlled via the `encoding` option which accepts
the following options:

| Encoding | Description |
| :------- | :---------- |
| `ndjson` | The payload will be encoded in new line delimited JSON payload, each line representing a JSON encoded event. |
| `text` | The payload will be encoded as new line delimited text, each line representing the value of the `"message"` key. |

### Health Checks

If the `healthcheck_uri` option is provided, Vector will issue a request
to this URI to determine the service's health before initializing the sink.
This ensures that the service is reachable. You can require this check with
the `--require-healthy` flag upon [starting][docs.starting] Vector.

### Rate Limiting

Vector offers a few levers to control the rate and volume of requests to the
downstream service. Start with the `rate_limit_duration` and `rate_limit_num`
options to ensure Vector does not exceed the specified number of requests in
the specified window. You can further control the pace at which this window is
saturated with the `request_in_flight_limit` option, which will guarantee no
more than the specified number of requests are in-flight at any given time.

Please note, Vector's defaults are carefully chosen and it should be rare that
you need to adjust these. If you found a good reason to do so please share it
with the Vector team by [opening an issie][url.new_http_sink_issue].

### Retry Policy

Vector will retry failed requests (status == `429`, >= `500`, and != `501`).
Other responses will _not_ be retried. You can control the number of retry
attempts and backoff rate with the `retry_attempts` and `retry_backoff_secs` options.

### Timeouts

To ensure the pipeline does not halt when a service fails to respond Vector
will abort requests after `30 seconds`.
This can be adjsuted with the `request_timeout_secs` option.

It is highly recommended that you do not lower value below the service's
internal timeout, as this could create orphaned requests, pile on retries,
and result in deuplicate data downstream.

### Authentication

HTTP authentication is controlled via the `Authorization` header which you can
set with the `headers` option. For convenience, Vector also supports the
`basic_auth.username` and `basic_auth.password` options which handle setting the
`Authorization` header for the [base access authentication
scheme][url.basic_auth].


## Troubleshooting

The best place to start with troubleshooting is to check the
[Vector logs][docs.monitoring_logs]. This is typically located at
`/var/log/vector.log`, then proceed to follow the
[Troubleshooting Guide][docs.troubleshooting].

If the [Troubleshooting Guide][docs.troubleshooting] does not resolve your
issue, please:

1. Check for any [open sink issues][url.http_sink_issues].
2. [Search the forum][url.search_forum] for any similar issues.
2. Reach out to the [community][url.community] for help.

## Resources

* [**Issues**][url.http_sink_issues] - [enhancements][url.http_sink_enhancements] - [bugs][url.http_sink_bugs]
* [**Source code**][url.http_sink_source]


[docs.at_least_once_delivery]: ../../../about/guarantees.md#at-least-once-delivery
[docs.config_composition]: ../../../usage/configuration/README.md#composition
[docs.event]: ../../../about/data-model.md#event
[docs.guarantees]: ../../../about/guarantees.md
[docs.log_event]: ../../../about/data-model.md#log
[docs.monitoring_logs]: ../../../usage/administration/monitoring.md#logs
[docs.sources]: ../../../usage/configuration/sources
[docs.starting]: ../../../usage/administration/starting.md
[docs.transforms]: ../../../usage/configuration/transforms
[docs.troubleshooting]: ../../../usage/guides/troubleshooting.md
[images.http_sink]: ../../../assets/http-sink.svg
[images.sink-flow-partitioned]: ../../../assets/sink-flow-partitioned.svg
[url.basic_auth]: https://en.wikipedia.org/wiki/Basic_access_authentication
[url.community]: https://vector.dev/community
[url.http_sink_bugs]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22Sink%3A+http%22+label%3A%22Type%3A+Bugs%22
[url.http_sink_enhancements]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22Sink%3A+http%22+label%3A%22Type%3A+Enhancements%22
[url.http_sink_issues]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22Sink%3A+http%22
[url.http_sink_source]: https://github.com/timberio/vector/tree/master/src/sinks/http.rs
[url.new_http_sink_issue]: https://github.com/timberio/vector/issues/new?labels%5B%5D=Sink%3A+http
[url.search_forum]: https://forum.vector.dev/search?expanded=true
[url.strftime_specifiers]: https://docs.rs/chrono/0.3.1/chrono/format/strftime/index.html
