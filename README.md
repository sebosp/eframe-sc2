# eframe sc2

[![dependency status](https://deps.rs/repo/github/sebosp/eframe-sc2/status.svg)](https://deps.rs/repo/github/sebosp/eframe-sc2)
[![Build Status](https://github.com/sebosp/eframe-sc2/workflows/CI/badge.svg)](https://github.com/sebosp/eframe-sc2/actions?workflow=CI)

This is a repo based on [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), a framework for writing apps using [egui](https://github.com/emilk/egui/).

Current status:
![Example screenshot](https://github.com/user-attachments/assets/aee9cda3-a572-429e-91cd-bad80b967074)

The backend runs as web server with axum and tokio.
It receives requests to interact with the polars datasets and serves HTML.

The frontend may run as native or in the browser, the eframe application interacts with the backend to pull information.

## Running the backend (axum)

```
$ export PATH=$HOME/.cargo/bin:/usr/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:$HOME/.fzf/bin
$ CARGO_TARGET_DIR=/tmp/s2proto-backend cargo watch -x clippy -x "run -- --source-dir $HOME/git/s2protocol-rs/ipcs/ -d -v debug"

```

This serves the front end as static files, the intention is to proxy the frontend as well so that it can avoid CORS issues.

## Running the frontend for development (trunk)

```
$ export PATH=$HOME/.cargo/bin:/usr/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:$HOME/.fzf/bin
$ CARGO_TARGET_DIR=/tmp/trunk trunk serve --address 0.0.0.0 --proxy-insecure --proxy-backend http://localhost:3000/api/
```

This allows running requests from the frontend (wasm) to the backend over the same host:port, avoiding CORS related issues.
Proxied by trunk itself.

# Roadmap

## Data Generation workflow
Currently different commands are used to generate the snapshot from other repos, this can be unified.

1. Set Input and Output directories for the location of SC2Replay packs and destination directory with generated snapshot.
2. Button to generate the snapshot, effectively running what in [sebosp/s2protocol-rs](https://github.com/sebosp/s2protocol-rs/) repo is `cargo run -r -- -v error --timing --source /home/seb/SCReplaysOnNVMe --output /home/seb/git/s2protocol-rs/ipcs/ write-arrow-ipc --process-max-files 10000000 all`
3. Change the statistics/filters to only 1v1 (easier to navigate/generate stats)
4. Future: Scan for new files and either regenerate snapshot or notify snapshot needs regenerating

## Player vs Player workflow:

On this workflow, it's possible to select time range further filter the current view.

NOTE: For each statistic it should be possible to generate a "pop-out" so that a statistic can be shown as a little piece on its own, such as game heart, something friendly for casters to show by activating an obs view.

1. Select Two Players (currently only one player is selectable), assign color to each, divide display in 50% width each player column.
2. Generate graph/counters in buckets grouped in all time, last 360 days, last 180 days, last 30 days.
2.1. total games
2.2. win/loss count and %
2.3. graph of win/loss over time
2.4. top played maps
2.5. workers created/killed over time.
2.6. gas/minerals gathered over time.
3. Show average game duration over time.
4. Generate table with top maps for each player.
5. Allow map selection to further filter the 1v1 player.
6. Show most effective units for each player over time (See [sebosp/s2-polars-data-analysis](https://github.com/sebosp/s2-polars-data-analys))
8. Show average (maybe fastest) upgrade timing.
9. Show latest replays sorted by date, show duration and win/loss.
10. When a specific replay it's selected:
10.1. the statistics above are focused on the selected replay.
10.2. a link to open the Replay Viewer on the browser should be engaged (See [swarmy](https://github.com/sebosp/swarmy/)
    This allows for previewing multiple games in different tabs to search for them.
    basically we start a viewer on the background and generate a link from here )
10.3. End army composition (top unit groups).
10.4. Show small map with units died and expansions
10.5. The chat messages are shown for this last replay and timing.
13. Future: It is possible to see the build order by capturing certain UnitDone/Initialized events and grouping by them, thus showing usual upgrades vs each other.
14. Future: maybe buckets of all time, last 4 months, last month.
15. Future: Integrate rerun-cli viewer, dunno if we can bundle it here so that the binary is available when the application is ran?
16. Future: MMR lost/won over time to each other.

## In Progress:

1. Select Two Players:
- Problem: Players are selected by "name", however a player may have multiple accounts and result in multiple "player_toon_id". Also other players may decide to use tha same "name" string and the stats would look broken.
- Current patch: a player name (without the clan or ID) will be used to group them together and allow selection. If the replay dataset contains garbage, it's hard to keep it clean from the very start.
- Future: Allow some sort of "well-known-ids" (maybe a file) where top players are known and configured/grouped and then one can use these well-knotwn-ids rather than Strings.
- 

