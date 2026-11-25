# Permissions and visibility

SysGhost runs as a normal user and needs no extra privileges for what it collects.

## What is read

- `/proc` is listed for numeric directory names.
- `/proc/<pid>/stat` is read for each one. This file is world-readable on a default Linux system.
- System CPU and memory are read through `sysinfo`.

## What is not read

Command lines, environment variables, open files, network sockets and memory contents are never opened.

## What can hide a process

| Cause | Effect |
| --- | --- |
| `hidepid=1` or `2` on the `/proc` mount | Other users' processes are hidden or unreadable, so they are skipped |
| A separate PID namespace (containers, `unshare`) | Only that namespace's processes are visible |
| A process exiting during the scan | It is skipped silently |

Unreadable or malformed entries are skipped, never fatal. The dashboard therefore shows what the current user can see, which may be fewer processes than `ps -e` run as root.

## Zombies

A zombie stays in `/proc` until its parent reaps it, so it keeps appearing with the state Zombie.
