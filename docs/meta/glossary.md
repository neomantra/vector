# Glossary

## Batch

"Batch" refers to a [batched payload](../about/data-model.md#batched-payload) within a sink. It is a batch of events encoded into a payload that the downstream service understands.

## Benchmark

"Benchmark" refers to a test designed to measure performance and resource usage. You can learn more about Vector's benchmarks in the [Benchmarks](../performance.md) section.

## Binary

"Binary" refers to the static binary that Vector compiles to. 

## Buffer

"Buffer" refers to an ordered queue of events that is coupled with a sink.

## Configuration

"Configuration" refers to the settings and options used to control Vector's behavior. You can learn more about Vector's configuration in the [Configuration](../usage/configuration/) section.

## Durability

"Durability" refers to the ability to retain data across exceptional events. In the context of Vector, this typically refers to the ability to retain data across restarts.

## Event

"Event" refers to a single unit of data that flows through Vector. You can learn more about events in the [Data Model](../about/data-model.md) section.

## Filter

"Filter" refers to a type of [transform](../usage/configuration/transforms/) that filters events or fields on an event.

## Flush

"flush" refers to the act of sending a batched payload to a downstream service. It is a common term used in conjunction with "buffer".

## Github

"[Github](https://github.com/)"" refers to the service used to host Vector's source code.

## Guide

"Guide" is a tutorial or walk through on a specific subject. You can see Vector's guides in the [Guides](../usage/guides/) section.

## Log

"Log" refers to an individual log event. This is a type of
[Vector event][docs.metric_event].

## Metric

"Metric" refers to an individual data unit used to represent a point in time
measurement. This is a type of [Vector event][docs.metric_event].

## Parser

"Parser" refers to a [transform][docs.transforms] that parses event data.

## Pipeline

"Pipeline" refers to the end result from combining [sources][docs.sources],
[transforms][docs.transforms], and [sinks][docs.sinks].

## Platform

"Platform" refers to a [platform][docs.platforms] that Vector can be deployed
on, such as [Docker][docs.docker].

## Reducer

"Reducer" refers to a [transform][docs.transforms] that reduces data into
a metric.

## Repo

"Repo" refers to a Git respository, usually the
[Vector Git repository][url.vector_repo].

## Role

"Role" refers to a [deployment role][docs.roles] that Vector is deployed
under.

## Router

"Router" refers is something that accepts and routes data to many destinations,
this is commonly used to describe Vector.

## Rust

"Rust" refers to the [Rust programming language][url.rust] that Vector is
written in.

## Sampler

"Sampler" refers to a [transform][docs.transforms] that samples data.

## Sink

"Sink" refers to the Vector [sink concept][docs.sinks].

## Source

"Source" refers to the Vector [source concept][docs.sources].

## Structured Log

"Structured log" refers to a log represented in a structured form, such as
a map. This is different from a text log which is represented as a single
text string.

## Table

"Table" refers to the [TOML table type][url.toml_table].

## TOML

"TOML" refers to [Tom's Obvious Markup Language][url.toml] and it is the syntax
used to represent the Vector configuration.

## Topology

"Topology" refers to a [deploy topology][docs.topologies] that Vector is
deployed under.

## Transform

"Transform" refers to the Vector [transform concept][docs.transforms].

## Use Case

"Use case" refers to a way in which Vector is used, such logs, metrics,
reducing cost, etc.

## Vector

"Vector" is the name of this project.


[docs.docker]: ../setup/installation/platforms/docker.md
[docs.metric_event]: ../about/data-model.md#metric
[docs.platforms]: ../setup/installation/platforms
[docs.roles]: ../setup/deployment/roles
[docs.sinks]: ../usage/configuration/sinks
[docs.sources]: ../usage/configuration/sources
[docs.topologies]: ../setup/deployment/topologies.md
[docs.transforms]: ../usage/configuration/transforms
[url.rust]: https://www.rust-lang.org/
[url.toml]: https://github.com/toml-lang/toml
[url.toml_table]: https://github.com/toml-lang/toml#table
[url.vector_repo]: https://github.com/timberio/vector
