# tfphl

`tfphl` is a high-performance terminal hockey dashboard for the **Federal Prospects Hockey League (FPHL)**, built with Rust and [Ratatui](https://github.com/ratatui/ratatui). It is modeled after [`tnhl`](https://github.com/rodeokevin/tnhl), bringing full terminal dashboard capabilities to FPHL fans, players, and broadcasters.

It interfaces directly with the official FPHL HockeyShift platform (`web.api.digitalshift.ca`) to deliver real-time scores, division standings, player/goalie leaderboards, boxscores, and play-by-play game logs.

---

## Features

- **Daily Games & Scores (`1`)**:
  - Live and final game scoreboards with start times, current period status, and venue.
  - Interactive date travel (`h` / `l` or Left / Right arrows) across the entire 2026-2027 season.
  - Jump to today's action anytime with `t`.
- **Division Standings (`2`)**:
  - Full division breakdown (Atlantic, Continental, Eastern, Western).
  - Metrics: Rank, Team, Games Played (GP), Wins (W), Losses (L), Overtime Losses (OTL), Points (PTS), Win% (W%), Goals For (GF), Goals Against (GA), Goal Differential (DIFF), Penalty Minutes (PIM), Last 10 (L10), and Current Streak.
  - Highlights your favorite team across all tables.
- **League Leaders (`3`)**:
  - Top Skaters: Rank, Name, Jersey #, Team, Position, GP, Goals, Assists, Points, Points Per Game (PPGA), +/-, PIM, Game Winning Goals (GWG), Shots on Goal (SOG).
  - Top Goalies: Rank, Name, Team, GP, Wins, Losses, OTL, Goals Against Average (GAA), Save Percentage (SV%), Shutouts (SO).
  - Quick toggle between Skaters and Goalies with `Tab` or `s` / `g`.
- **Boxscore & Play-by-Play Modal (`Enter`)**:
  - Period-by-period scoring breakdown (1st, 2nd, 3rd, OT, Total).
  - Team shot totals and power play efficiency comparisons.
  - Complete chronological play-by-play event feed (Goals, Shots, Penalties, Goalie changes).
- **Built-in Help Screen (`?`)**:
  - Instant modal keymap showing all available keyboard shortcuts.

---

## Keyboard Controls

| Key | Action |
| --- | --- |
| `1` | Switch to **Games** view |
| `2` | Switch to **Standings** view |
| `3` | Switch to **Leaders** view |
| `h` / `◄` | Navigate to **previous game date** |
| `l` / `►` | Navigate to **next game date** |
| `t` | Jump to **today** |
| `j` / `▼` | Select **next** game / item |
| `k` / `▲` | Select **previous** game / item |
| `Enter` | Open **Boxscore & Play-by-Play** for selected game |
| `Tab` / `s` / `g` | Toggle between **Skaters** and **Goalies** in Leaders view |
| `r` | Force refresh data from HockeyShift API |
| `?` | Toggle **Help** screen |
| `Esc` / `q` | Close modal / Exit application |
| `Ctrl + C` | Force quit |

---

## Installation & Running

### Requirements
- [Rust toolchain](https://rustup.rs) (version 1.88 or newer)

### Build and Run

```bash
git clone https://github.com/ctuchik/tfphl.git # or local folder
cd ~/Projects/tfphl
cargo run --release
```

---

## Configuration

`tfphl` reads an optional configuration file located at:
- Linux: `~/.config/tfphl/tfphl.toml`

The file is automatically created with sensible defaults on first run:

```toml
favorite_team = "BBB"      # Team abbreviation to highlight (e.g. BBB, MCR, IND, TCT)
timezone = "US/Eastern"   # Timezone for game times
log_level = "error"       # Logging verbosity
```

### Supported FPHL Teams & Codes
- `BKF` — Baton Rouge Kingfish
- `BBB` — Binghamton Black Bears
- `BRB` — Blue Ridge Bobcats
- `TCT` — Twin City Thunderbirds
- `DHT` — Danbury Hat Tricks
- `MMC` / `MRM` — Monroe Moccasins
- `MSM` — Mid-South Monarchs
- `MNL` — Minnesota Northern Lights
- `PHP` — Port Huron Prowlers
- `CRD` — Columbus River Dragons
- `MCR` — Motor City Rockers
- `TOP` — Topeka Scarecrows
- `IND` — Indiana Sentinels
- `OCS` — Oceanside Shock
- `SCK` — Stockton Thunder
- `WTW` — Watertown Wolves
- `FRF` — Fresno Falcons
