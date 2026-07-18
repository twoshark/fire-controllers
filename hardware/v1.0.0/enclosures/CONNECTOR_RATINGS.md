# Connector ratings (revised)

Loads: **2 A/ch** · sign **10 A** · mp **6 A** · LRS **17 A** max · PCB ceiling **16 A**.

| Interface | Part | Rating | Load | Seal | Joint | Verdict |
| --- | --- | --- | ---: | --- | --- | --- |
| 12 V panel/field | DTP04-2P / DTP06-2S (size 12) | **25 A** · 12–14 AWG | ≤10 A (sign) / 6 A (mp) | **IP67 mated face**; panel via printed pocket | Crimp | **OK** |
| SOL | DT 2-pin (JRready) | 13 A · 14–18 AWG | 2 A | **IP67 mated face**; panel via printed pocket | Crimp | **OK** |
| RS-485 | M12-5 | 4 A · ≤22 AWG | signal | **IP67 mated** | Solder/field | **OK** |
| POWER (AC boxes only) | KCD4 DPST + boot | 20 A / 125 **VAC** | ~2 A AC | splash w/ boot | QC | **OK on AC only** |
| AC inlet | C14 + cover | 10–15 A | ~2 A | cover when unmated | QC | **OK** |
| Fans | 60 mm 12 V | ≪0.2 A ea | on LRS DC | filtered open | splice | **OK** (not IP67) |

### Rejected / removed

| Was | Why |
| --- | --- |
| PanelPole2 + PP30 | Current OK (30 A) but **not IP67 when mated**; $80 for 4 panels |
| KCD4 on sign/mp-output (12 V) | **AC-only** rating — unsafe assumption at 10 A DC |
| WRG32 as DC fix | Also no agency DC rating |

### PCB path notes

| Item | Note |
| --- | --- |
| `J1` / `J6` KF128-7.5 | Use genuine KEFA C474954 (24 A). Parallel **both** `J6` poles for load +12 V |
| SOL +12 V | **Star** from `J6` with 12 AWG stub → 18 AWG per DT pin1 (do not daisy-chain 18 AWG for 10 A) |
| `F9` 25 A ATO | Catastrophic/reverse; channel PTCs limit load. Does not coordinate with LRS 17 A |
| Kill switch | AC KCD4 on matching `*-output-power` removes 12 V from output box |

PCB terminals remain screw. No panel USB / RESET / BOOT. Sealing details: [`SEALING.md`](SEALING.md).
