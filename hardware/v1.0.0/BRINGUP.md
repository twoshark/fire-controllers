# Board Bring-Up Guide

Checklist for first power-on of the 2026-07-15 v1.0.0 boards.
Pin map: [`PIN_MAP.md`](PIN_MAP.md).
As-built parts: [`INPUT_BOARD.md`](INPUT_BOARD.md) / [`OUTPUT_BOARD.md`](OUTPUT_BOARD.md).

## 0) Tools you need

| Tool | Purpose |
| --- | --- |
| Multimeter | Rails, continuity |
| ST-Link / J-Link / CMSIS-DAP | SWD flash + RTT (recommended) |
| USB-C cable | DFU fallback |
| 12V bench supply (current-limited) | Safer than enclosure PSU for first bring-up |
| Optional: second 12V supply / solenoids | Load tests after link works |

Host packages:

```bash
# Rust (nightly via firmware/rust-toolchain.toml)
# probe-rs for SWD
cargo install probe-rs-tools --locked

# DFU fallback
sudo apt install dfu-util   # or brew install dfu-util
```

## 1) Pre-power visual checks

### Both boards

- [ ] No solder bridges on MCU / USB-C / RS-485 SOICs
- [ ] Polarized caps oriented to silkscreen `+` (`C17`/`C18`/`C28` input; `C17`/`C18`/`C19`/`C30`/`C35` output)
- [ ] `SW1` = reset, `SW2` = BOOT0 labeled / reachable

### Output board only

- [ ] `F9` blade fuse **inserted** (**20 A** ATO) before applying load current
- [ ] `Q9` (TO-263) present between fuse and bulk caps
- [ ] HLG-185H-12: FG earthed; DC → DTP → `J1`; mount shaded

## 2) Cold rail bring-up (no firmware yet)

Apply **current-limited** 12V to `J1` (start ~200mA limit on input board, ~500mA on output with no loads).

| Step | Measure | Pass |
| --- | --- | --- |
| 1 | `J1` polarity correct | — |
| 2 | `12V_MAIN` ~12V | Within ~0.5V of supply |
| 3 | `3V3` at MCU decoupling | 3.20 .. 3.40V |
| 4 | Power LED on | `LED1` / `PWR` lit |
| 5 | `PA14`/BOOT0 idle | ~0V (pulldown) |
| 6 | `NRST` idle | ~3.3V |

If `3V3` is wrong: stop. Check `U4`/`L1` and 3V3 bulk caps before attaching a probe
(input: `C18`/`C28`; output: `C19`/`C30`).

## 3) Flash firmware (SWD preferred)

```bash
# From repo root
./scripts/test.sh          # host tests + cross-compile
./scripts/flash-input.sh   # builds + probe-rs run (leaves RTT attached)
# or, in another terminal after flash:
./scripts/flash-output.sh
```

Expected RTT / defmt line:

- input: `input-controller: init complete`
- output: `output-controller: init complete`

DFU fallback (no probe):

```bash
./scripts/dfu-flash-input.sh
./scripts/dfu-flash-output.sh
```

DFU entry: hold `SW2` (BOOT0) -> tap `SW1` (NRST) -> release `SW2`.

## 4) Single-board functional tests

### Input board alone

| Test | Action | Expect |
| --- | --- | --- |
| Channel LEDs | Short each `J2a`/`J2b` pin to `J3` GND | Matching yellow LED lights |
| Link LED | No RS-485 peer | `LED2` blinks (~5Hz) |
| TX activity | Scope / logic on `PA9` or `CN2` TX pair | Edge TX + ~40 Hz idle keepalive |

### Output board alone

| Test | Action | Expect |
| --- | --- | --- |
| Failsafe | Power with no serial peer | `LINK` fast-blink; all gates OFF |
| Override | Short `J3a.1` to `J4` GND | `CH0` LED on; `J5a.1` pulled low (with load or meter) |
| Heartbeat TX | Scope `PA9` / `J2` TX | ~10Hz frames |

**Load wiring:** `J6` (+12V) -> load -> `J5x.k`. Never return load to `J1` GND.

## 5) Link test (both boards)

1. Wire crossover Belden 9842 (or equiv) per pin map:
   - Input `CN2` TX+/- -> Output `J2` RX+/-
   - Input `CN2` RX+/- -> Output `J2` TX+/-
   - GND to GND; shield per enclosure policy
2. Power both boards.
3. Expect:
   - Input `LED2` solid (heartbeat received)
   - Output `LINK` solid (state frames received; not failsafe)
4. Close input CH0 -> output `CH0` LED + gate follow within a few ms.
5. Open input, assert output override -> output stays ON.
6. Unplug RS-485 -> failsafe within ~100ms (`LINK` blinks; serial-driven channels OFF). Assert an override -> that channel still turns ON.

## 6) Script cheat sheet

| Script | What it does |
| --- | --- |
| `./scripts/build.sh` | Release-build both firmware images |
| `./scripts/test.sh` | Protocol unit tests + cross-compile |
| `./scripts/flash-input.sh` | SWD flash input + attach |
| `./scripts/flash-output.sh` | SWD flash output + attach |
| `./scripts/dfu-flash-input.sh` | USB DFU flash input |
| `./scripts/dfu-flash-output.sh` | USB DFU flash output |
| `./scripts/monitor.sh` | Attach probe-rs RTT without reflashing |

## 7) Known v1.0.0 gotchas

- Input CH6/CH7 are on `PB0`/`PB1` (not `PA6`/`PA7`); firmware polls them at 1 ms because EXTI0/1 are used by CH0/CH1.
- Input CH0 LED is `PB10`, not `PB2`.
- Both boards: `U2A`=TX, `U2B`=RX.

- Input RS-485 connector is **`CN2`**, not `J4`.
- Output reverse protection is **`Q9` P-MOS**, not a series SS34.
- Output PTCs are **`1812L200/16GR`** (16V).
- No series gate resistors on output v1.0.0 — gates are direct + 10k PD.
