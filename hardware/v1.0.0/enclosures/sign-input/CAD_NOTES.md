# sign-input — CAD index

**Print two parts:** [`BODY.md`](BODY.md) (box) · [`LID.md`](LID.md) (lid).  
All XY in those files are **outer-base** (0,0)=front-left; steps are sketch → extrude/cut.  
Workflow: [`../CAD_PARTS.md`](../CAD_PARTS.md). Refs: [`../MOUNTING.md`](../MOUNTING.md) · [`../PANEL_CUTOUTS.md`](../PANEL_CUTOUTS.md).

| Export | Role |
| --- | --- |
| `sign-input-body.stl` | Open-top box · panels · PCB/RS-15 bosses |
| `sign-input-lid.stl` | Lid · gasket · hinges · hex arcade |

| Dim | mm |
| --- | ---: |
| Outer | **220 × 180 × 90** |
| Inner | **214 × 174 × 84** |

Bed ≤ **256 × 256**. Body open-up; lid groove-up; DT/M12 retainers = separate clips.
