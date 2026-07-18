# Connector ratings

Loads: **2 A/ch** · sign **10 A** · mp **6 A** · HLG **16 A** · PCB ceiling **16 A**.

| Interface | Part | Rating | Load | Seal | Joint | Verdict |
| --- | --- | --- | ---: | --- | --- | --- |
| 12 V | DTP04-2P / DTP06-2S | **25 A** · 12–14 AWG | ≤10 / 6 A | IP67 mated face; printed pocket | Crimp | **OK** |
| SOL | DT 2-pin | 13 A · 14–18 AWG | 2 A | IP67 mated face; printed pocket | Crimp | **OK** |
| RS-485 | M12-5 | 4 A · ≤22 AWG | signal | IP67 mated | Solder/field | **OK** |
| POWER (inputs) | KCD4 DPST + boot | 20 A / 125 **VAC** | ~0.2 A AC | splash w/ boot | QC | **OK on AC only** |
| AC inlet (inputs) | C14 + cover | 10–15 A | ≪1 A | cover when unmated | QC | **OK** |
| Outdoor PSU | HLG-240H-12 | 16 A @ 12 V | ≤10 / 6 A | **IP67 unit** | factory cable | **OK** |

### PCB notes

| Item | Note |
| --- | --- |
| `J1` / `J6` KF128-7.5 | Genuine KEFA C474954 (24 A). Parallel **both** `J6` poles |
| SOL +12 V | **Star** from `J6` · 12 AWG stub → 18 AWG/ch |
| `F9` 25 A ATO | Catastrophic/reverse; channel PTCs limit load |
| Kill 12 V | Unplug HLG AC cord |

No panel USB / RESET / BOOT. Seal detail: [`SEALING.md`](SEALING.md).
