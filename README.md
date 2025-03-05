# Hat and HatBlocks repo

Welcome to the **Hat Monorepo**, the unified repository for the **Hat** hybrid DSL system. This repository contains the core components for the **Hat DSL**, a hybrid textual and visual domain-specific language (DSL) for smart home automation, as well as the research data collected for its development.

## Repository Structure

```
hat-monorepo/
│── hat/               # The Hat DSL core implementation (Rust)
│── hatblocks/         # The HatBlocks visual programming website (TypeScript/Astro)
│── research-data/     # Data collected from research questionnaires
│── README.md          # This document
│── LICENSE            # Licensing information
│── .gitignore         # Git ignore file
```

---

## Hat DSL (`hat/`)

### Example:
```rust
automation "Night Light" (MotionSensorOnEvent) {
    if get_device() == "BedroomSensor"
    if event_time_between("22:00", "06:00")
    run turn_on_device("BedroomLight")
}
```

### Installation & Usage:
To build and run Hat locally:
```sh
cd hat
cargo build
cargo run
```

### Environment Variables:
Hat uses the following environment variables:

| Variable   | Description |
|------------|-------------|
| `RUST_LOG` | Log level (`debug`, `info`, `warn`, `error`). Default: `info` |
| `HA_URL`   | Home Assistant URL for integration |
| `HA_TOKEN` | Home Assistant authentication token |

To set these variables before running Hat, use:

```sh
export RUST_LOG=info
export HA_URL="http://homeassistant.local:8123"
export HA_TOKEN="your_home_assistant_token"
```

---

## HatBlocks (`hatblocks/`)

### Running the HatBlocks Website:
```sh
cd hatblocks
npm install
npm run dev
```
Then, open the URL that will appear in the output of the command in your browser.

### Environment Variables:
HatBlocks requires the following environment variable:

| Variable              | Description |
|-----------------------|-------------|
| `PUBLIC_HAT_ENDPOINT` | HTTP endpoint for the Hat REST API |

To set this variable, use:
```sh
export PUBLIC_HAT_ENDPOINT="http://localhost:8000"
```

---

## Research Data (`research-data/`)
This folder contains **data collected from usability studies and questionnaires** conducted as part of the research behind Hat and HatBlocks. The data includes:
- **User study responses**
- **Prototype feedback**

