---
description: Accepts `log` events and allows you to parse a field's value with a Regular Expression.
---

<!--
     THIS FILE IS AUTOOGENERATED!

     To make changes please edit the template located at:

     scripts/generate/templates/docs/usage/configuration/transforms/regex_parser.md.erb
-->

# regex_parser transform

![][images.regex_parser_transform]


The `regex_parser` transform accepts [`log`][docs.log_event] events and allows you to parse a field's value with a [Regular Expression][url.regex].

## Config File

{% code-tabs %}
{% code-tabs-item title="vector.toml (example)" %}
```coffeescript
[sinks.my_regex_parser_transform_id]
  # REQUIRED - General
  type = "regex_parser" # must be: "regex_parser"
  inputs = ["my-source-id"]
  regex = "^(?P<host>[\\w\\.]+) - (?P<user>[\\w]+) (?P<bytes_in>[\\d]+) \\[(?P<timestamp>.*)\\] \"(?P<method>[\\w]+) (?P<path>.*)\" (?P<status>[\\d]+) (?P<bytes_out>[\\d]+)$"

  # OPTIONAL - General
  drop_failed = false # default
  drop_field = true # default
  field = "message" # default

  # OPTIONAL - Types
  [sinks.my_regex_parser_transform_id.types]
    * = {name = "status", value = "int"} # no default, enum: "string", "int", "float", "bool", "timestamp|strftime"
```
{% endcode-tabs-item %}
{% code-tabs-item title="vector.toml (schema)" %}
```coffeescript
[sinks.<sink-id>]
  # REQUIRED - General
  type = "regex_parser"
  inputs = ["<string>", ...]
  regex = "<string>"

  # OPTIONAL - General
  drop_failed = <bool>
  drop_field = <bool>
  field = "<string>"

  # OPTIONAL - Types
  [sinks.<sink-id>.types]
    * = {"string" | "int" | "float" | "bool" | "timestamp|strftime"}
```
{% endcode-tabs-item %}
{% code-tabs-item title="vector.toml (specification)" %}
```coffeescript
[sinks.regex_parser]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "regex_parser"
  type = "regex_parser"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The Regular Expression to apply. Do not inlcude the leading or trailing `/`.
  # 
  # * required
  # * no default
  regex = "^(?P<host>[\\w\\.]+) - (?P<user>[\\w]+) (?P<bytes_in>[\\d]+) \\[(?P<timestamp>.*)\\] \"(?P<method>[\\w]+) (?P<path>.*)\" (?P<status>[\\d]+) (?P<bytes_out>[\\d]+)$"

  # If `true`, events that fail to properly parse will be dropped.
  # 
  # * optional
  # * default: false
  drop_failed = false

  # If the `field` should be dropped (removed) after parsing.
  # 
  # * optional
  # * default: true
  drop_field = true

  # The field to parse.
  # 
  # * optional
  # * default: "message"
  field = "message"

  #
  # Types
  #

  [sinks.regex_parser.types]
    # A definition of mapped field types. They key is the field name and the value
    # is the type. `strftime` specifiers are supported for the `timestamp` type.
    # 
    # * optional
    # * no default
    # * enum: "string", "int", "float", "bool", "timestamp|strftime"
    status = "int"
    duration = "float"
    success = "bool"
    timestamp = "timestamp|%s"
    timestamp = "timestamp|%+"
    timestamp = "timestamp|%F"
    timestamp = "timestamp|%a %b %e %T %Y"
```
{% endcode-tabs-item %}
{% endcode-tabs %}

## Options

| Key  | Type  | Description |
|:-----|:-----:|:------------|
| **REQUIRED** - General | | |
| `type` | `string` | The component type<br />`required` `enum: "regex_parser"` |
| `inputs` | `[string]` | A list of upstream [source][docs.sources] or [transform][docs.transforms] IDs. See [Config Composition][docs.config_composition] for more info.<br />`required` `example: ["my-source-id"]` |
| `regex` | `string` | The Regular Expression to apply. Do not inlcude the leading or trailing `/`.<br />`required` `example: (see above)` |
| **OPTIONAL** - General | | |
| `drop_failed` | `bool` | If `true`, events that fail to properly parse will be dropped.<br />`default: false` |
| `drop_field` | `bool` | If the `field` should be dropped (removed) after parsing.<br />`default: true` |
| `field` | `string` | The field to parse.<br />`default: "message"` |
| **OPTIONAL** - Types | | |
| `*` | `string` | A definition of mapped field types. They key is the field name and the value is the type. [`strftime` specifiers][url.strftime_specifiers] are supported for the `timestamp` type.<br />`no default` `enum: "string", "int", "float", "bool", "timestamp\|strftime"` |

## Examples

Given the following log line:

{% code-tabs %}
{% code-tabs-item title="log" %}
```json
{
  "message": "5.86.210.12 - zieme4647 5667 [19/06/2019:17:20:49 -0400] \"GET /embrace/supply-chains/dynamic/vertical\" 201 20574"
}
```
{% endcode-tabs-item %}
{% endcode-tabs %}

And the following configuration:

{% code-tabs %}
{% code-tabs-item title="vector.toml" %}
```coffeescript
[transforms.<transform-id>]
  type = "regex_parser"
  field = "message"
  regex = '^(?P<host>[\w\.]+) - (?P<user>[\w]+) (?P<bytes_in>[\d]+) \[(?P<timestamp>.*)\] "(?P<method>[\w]+) (?P<path>.*)" (?P<status>[\d]+) (?P<bytes_out>[\d]+)$'

[transforms.<transform-id>.types]
  bytes_int = "int"
  timestamp = "timestamp|%m/%d/%Y:%H:%M:%S %z"
  status = "int"
  bytes_out = "int"
```
{% endcode-tabs-item %}
{% endcode-tabs %}

A [`log` event][docs.log_event] will be emitted with the following structure:

```javascript
{
  // ... existing fields
  "bytes_in": 5667,
  "host": "5.86.210.12",
  "user_id": "zieme4647",
  "timestamp": <19/06/2019:17:20:49 -0400>,
  "message": "GET /embrace/supply-chains/dynamic/vertical",
  "status": 201,
  "bytes": 20574
}
```

Things to note about the output:

1. The `message` field was overwritten.
2. The `bytes_in`, `timestamp`, `status`, and `bytes_out` fields were coerced.


## How It Works

## Types
You can coerce your extract values into types via the `types` table
as shown in the examples above. The supported types are:

| Type | Desription |
| :--- | :--------- |
| `string` | Coerces to a string. Generally not necessary since values are extracted as strings. |
| `int` | Coerce to a 64 bit integer. |
| `float` | Coerce to 64 bit floats. |
| `bool`  | Coerces to a `true`/`false` boolean. The `1`/`0` and `t`/`f` values are also coerced. |

### Failed Parsing

If the `field` value fails to parse against the provided `regex` then an error
will be [logged][docs.monitoring_logs] and the event will be kept or discarded
depending on the `drop_failed` value.

A failure includes any event that does not successfully parse against the
provided `regex`. This includes bad values as well as events missing the
specified `field`.

### Performance

The `regex_parser` source has been involved in the following performance tests:

* [`regex_parsing_performance`][url.regex_parsing_performance_test]

Learn more in the [Performance][docs.performance] sections.

### Regex Debugger

To test the validity of the `regex` option, we recommend the [Golang Regex
Tester][url.regex_tester] as it's Regex syntax closely 
follows Rust's.

### Regex Syntax

Vector follows the [documented Rust Regex syntax][url.rust_regex_syntax] since
Vector is written in Rust. This syntax follows a Perl-style regular expression
syntax, but lacks a few features like look around and backreferences.

#### Named Captures

You can name Regex captures with the `<name>` syntax. For example:

```
^(?P<timestamp>.*) (?P<level>\w*) (?P<message>.*)$
```

Will capture `timestamp`, `level`, and `message`. All values are extracted as
`string` values and must be coerced with the `types` table.

More info can be found in the [Regex grouping and flags
documentation][url.regex_grouping_and_flags].

#### Flags

Regex flags can be toggled with the `(?flags)` syntax. The available flags are:

| Flag | Descriuption |
| :--- | :----------- |
| `i`  | case-insensitive: letters match both upper and lower case |
| `m`  | multi-line mode: ^ and $ match begin/end of line |
| `s`  | allow . to match `\n` |
| `U`  | swap the meaning of `x*` and `x*?` |
| `u`  | Unicode support (enabled by default) |
| `x`  | ignore whitespace and allow line comments (starting with `#`)

For example, to enable the case-insensitive flag you can write:

```
(?i)Hello world
```

More info can be found in the [Regex grouping and flags
documentation][url.regex_grouping_and_flags].


## Troubleshooting

The best place to start with troubleshooting is to check the
[Vector logs][docs.monitoring_logs]. This is typically located at
`/var/log/vector.log`, then proceed to follow the
[Troubleshooting Guide][docs.troubleshooting].

If the [Troubleshooting Guide][docs.troubleshooting] does not resolve your
issue, please:

1. Check for any [open sink issues][url.regex_parser_transform_issues].
2. [Search the forum][url.search_forum] for any similar issues.
2. Reach out to the [community][url.community] for help.


### Alternatives

Finally, consider the following alternatives:


* [`grok_parser` transform][docs.grok_parser_transform]

* [`lua` transform][docs.lua_transform]

* [`tokenizer` transform][docs.tokenizer_transform]

## Resources

* [**Issues**][url.regex_parser_transform_issues] - [enhancements][url.regex_parser_transform_enhancements] - [bugs][url.regex_parser_transform_bugs]
* [**Source code**][url.regex_parser_transform_source]
* [**Regex Tester**][url.regex_tester]
* [**Rust Regex Syntax**][url.rust_regex_syntax]


[docs.config_composition]: ../../../usage/configuration/README.md#composition
[docs.grok_parser_transform]: ../../../usage/configuration/transforms/grok_parser.md
[docs.log_event]: ../../../about/data-model.md#log
[docs.lua_transform]: ../../../usage/configuration/transforms/lua.md
[docs.monitoring_logs]: ../../../usage/administration/monitoring.md#logs
[docs.performance]: ../../../performance.md
[docs.sources]: ../../../usage/configuration/sources
[docs.tokenizer_transform]: ../../../usage/configuration/transforms/tokenizer.md
[docs.transforms]: ../../../usage/configuration/transforms
[docs.troubleshooting]: ../../../usage/guides/troubleshooting.md
[images.regex_parser_transform]: ../../../assets/regex_parser-transform.svg
[url.community]: https://vector.dev/community
[url.regex]: https://en.wikipedia.org/wiki/Regular_expression
[url.regex_grouping_and_flags]: https://docs.rs/regex/1.1.7/regex/#grouping-and-flags
[url.regex_parser_transform_bugs]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22Transform%3A+regex_parser%22+label%3A%22Type%3A+Bugs%22
[url.regex_parser_transform_enhancements]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22Transform%3A+regex_parser%22+label%3A%22Type%3A+Enhancements%22
[url.regex_parser_transform_issues]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22Transform%3A+regex_parser%22
[url.regex_parser_transform_source]: https://github.com/timberio/vector/tree/master/src/transforms/regex_parser.rs
[url.regex_parsing_performance_test]: https://github.com/timberio/vector-test-harness/tree/master/cases/regex_parsing_performance
[url.regex_tester]: https://regex-golang.appspot.com/assets/html/index.html
[url.rust_regex_syntax]: https://docs.rs/regex/1.1.7/regex/#syntax
[url.search_forum]: https://forum.vector.dev/search?expanded=true
[url.strftime_specifiers]: https://docs.rs/chrono/0.3.1/chrono/format/strftime/index.html
