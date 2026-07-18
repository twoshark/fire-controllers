# sign-output-power — CAD

Orientation: [`../README.md`](../README.md). Cutouts: [`../PANEL_CUTOUTS.md`](../PANEL_CUTOUTS.md).

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **240 × 180 × 95** |
| Wall | 2.5–3.0 |
| Lid | openable |

Fans on **long** walls — LRS is **215** long; short ends are too narrow for fan + LRS in-axis.

## Sides

Mate face → sign-output at same height.

```text
12V MATE FACE (short):  [PanelPole2 OUT Ø28.6]  CL = 40  (centered)
AC END (short):         [C14] CL 35  [POWER KCD4] CL 50
LONG WALL A:            intake [filter] + [fan 60]  CL 48
LONG WALL B:            exhaust [grill] + [fan 60]  CL 48
```

| Feature | Cutout | CL height | Notes |
| --- | --- | ---: | --- |
| PanelPole2 OUT | **Ø28.6** | **40** | Rear depth 22.2 |
| C14 | 27.5×20 or 27×19 | **35** | — |
| POWER KCD4 | 30×22 | **50** | AC L |
| Fan airflow | **Ø57** | **48** | Frame 60×60; screws **50×50** / Ø3.5 |
| Filter / grill | match fan | **48** | intake + exhaust |

## Internal

| Item | Size / place |
| --- | --- |
| LRS-200-12 | **215 × 115 × 30** on Al **215 × 115 × 3** |
| LRS mount | M4 L≤5; hole span typ. **150** length / verify unit |
| Fans ×2 | on LRS +V/−V; airflow across long axis |
| Clearance above LRS | ≥15 to lid |

## Cables (external)

| Cable | Length |
| --- | ---: |
| C13→C14 mains | **6 ft** |
| Powerpole → sign-output | **≤ 4 ft** DIY 12 AWG |

LRS: selector **115 V**, trim **12.0 V**.
