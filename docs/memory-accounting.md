# Memory accounting

## System memory

`memory_used_bytes` and `memory_total_bytes` come from `sysinfo` (`used_memory()` and `total_memory()`). The dashboard shows `RAM used / total` in terminals where the gauge is at least 60 columns wide, and a percentage otherwise.

## Process memory

Per-process memory is the resident set size (RSS):

```
memory_bytes = rss_pages * page_size
```

`rss_pages` is field 24 of `/proc/<pid>/stat` and `page_size` comes from `rustix::param::page_size()`.

- Negative page counts are treated as 0.
- The multiplication saturates at `u64::MAX` instead of overflowing.
- RSS counts shared pages in every process that maps them, so per-process values can add up to more than system usage.
- The kernel's RSS figure is an estimate; it is not a precise accounting of private memory.

Display uses binary units (KiB, MiB, GiB, TiB). The details view also shows the exact byte count.
