# Rustic Log Furnace

**Rustic Log Furnace** is a high-performance, flexible log processing pipeline built with Rust. Designed for reliability and speed, its goal is to consume raw log data from multiple sources, apply a chain of powerful parsing and transformation operations, and route the resulting structured data to various sinks for analysis and storage.

## Overview

In the world of system monitoring and security analysis, getting unstructured log lines into a clean, searchable format is critical. Rustic Log Furnace aims to be the robust, efficient tool that sits between your log sources (files, network streams) and your storage layer (databases, files, queues).

The core philosophy is a modular, plug-and-play architecture defined by a declarative configuration: **Sources** feed data, **Parsers** transform it, and **Sinks** export the results.

## Current State (Minimum Viable Processor)

While the full vision (detailed below) is expansive, the current version of Rustic Log Furnace supports a limited, foundational feature set:

| Component | Currently Supported | Type |
| :--- | :--- | :--- |
| **Source** | Single File Monitoring | File Tail |
| **Sink** | Single File Output | File Append |
| **Parsers** | Field Extraction & Cleanup | Regex Split, Drop Field |

This means you can currently monitor a single log file, apply basic field extraction and cleanup, and continuously write the results to a single output file.

## Planned Configuration (Target)

The ultimate goal for Rustic Log Furnace is to support complex, multi-stage data processing defined by a comprehensive TOML configuration file.

**NOTE: Configuration via TOML is not yet implemented.** The structure below represents the **target architecture** and the full possible set of configurations can be found at [`resources/full_config.toml`](https://github.com/AthulyaWeerakoon/rustic-log-furnace/blob/main/resources/full_config.toml).

### Configuration Concept

The pipeline is organized around `[[processor]]` blocks, each defining an independent log processing workflow.

#### Sources (Planned)

Ability to pull data from diverse locations simultaneously.

| Protocol | Description |
| :--- | :--- |
| `file` | Tail a local file for continuous updates (current implementation). |
| `tcp` | Listen on a network port for incoming log streams. |

#### Parsers (Planned)

Data from all sources of a processor gets fed to all configured patterns of that processor, each of which has a fully ordered chain of parsers designed for granular control over data transformation.

| Parser Type | Description |
| :--- | :--- |
| `regex_split` | Splits an input field (`field_in`) using a regex, populating multiple new fields (`fields_out`). (Currently supported) |
| `drop_fields` | Removes specified fields from the log record. (Currently supported) |
| `time_simple` | Reformat and standardize timestamp fields. |
| `uppercase` | Convert field content to uppercase. |
| `count_pattern` | Counts occurrences of a regex pattern within a field. |
| `global_count` | Aggregates a count field across all processed logs (e.g., total failures). |
| `global_max` | Tracks the maximum value seen in a field across all logs (e.g., peak invalid attempts). |
| `global_latest` | Tracks the latest timestamp seen for a specific event. |
| `format_append` | Creates a new summary field based on a template string and existing fields. |

#### Sinks (Planned)

Routing processed data to different destinations, supporting both **local** (per-log-entry) and **global** (aggregate stats) scopes.

| Sink Type | Scope | Operation | Description |
| :--- | :--- | :--- | :--- |
| `json_file` | `local` | `append` | Appends individual processed log entries as JSON objects to a file. |
| `json_file` | `global` | `overwrite` | Periodically overwrites a file with current aggregate statistics (e.g., counters, max values). |

### Example Target Configuration

The following TOML snippet illustrates the planned capability to ingest authentication logs, process them through multiple transformation stages, and output both the processed line and global statistics.

```toml
##################################################
# PROCESSOR
##################################################

[[processor]]
name = "auth_logs_processor"
type = "tail"

  ##################################################
  # SOURCES (multiple)
  ##################################################

  [[processor.sources]]
  protocol = "file"
  path = "/var/log/auth.log"

  [[processor.sources]]
  protocol = "tcp"
  host = "0.0.0.0"
  port = 9001


  ##################################################
  # PATTERNS (one or more)
  ##################################################

  [[processor.patterns]]
  name = "ssh_failed_login"
  regex = '^(?P<date>\w{3} \d{1,2}) (?P<time>\d{2}:\d{2}:\d{2}) (?P<host>[\w\-]+) (?P<process>[^ ]+): (?P<msg>.*)$'


    ##################################################
    # PARSERS (ordered)
    ##################################################

    # Parser #1: Initial Field Split
    [[processor.patterns.parsers]]
    order = 1
    type = "regex_split"
    field_in = "line"
    regex = '^(?P<date>\w{3} \d{1,2}) (?P<time>\d{2}:\d{2}:\d{2}) (?P<host>[\w\-]+) (?P<process>[^ ]+): (?P<msg>.*)$'
    fields_out = ["date", "time", "host", "process", "msg"]
    keep_original = false

    # Parser #8: Drop unnecessary field
    [[processor.patterns.parsers]]
    order = 8
    type = "drop_fields"
    drop = ["host"]

    # ... (Other planned parsers like time formatting, global aggregation, etc., would follow here)

    ##################################################
    # SINKS (local + global)
    ##################################################

    [[processor.patterns.sinks]]
    name = "local_json_output"
    type = "json_file"
    path = "/tmp/auth_processed.json"
    operation = "append"
    scope = "local"

    [[processor.patterns.sinks]]
    name = "global_stats_output"
    type = "json_file"
    path = "/tmp/auth_aggregate.json"
    operation = "overwrite"
    scope = "global"
```

## Getting Started
### 1. Clone the repository
```bash
git clone https://github.com/AthulyaWeerakoon/rustic-log-furnace.git
cd rustic-log-furnace
```
### 2. Build the project
```bash
cargo build --release
```
### 3. Configure the furnace's toml (Currently not implemented)
### 4. Run the furnace
```bash
./target/release/LogProcessor

```
