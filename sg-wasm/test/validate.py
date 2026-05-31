import sys
from pathlib import Path
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.collections import PolyCollection
import trimesh

d = Path(__file__).parent
glb = d / "stained_glass_rust.glb"
scene = trimesh.load(glb)
print("=== GLB structure ===")
print("geometries:", len(scene.geometry))
tottri = 0
items = []
for name, g in scene.geometry.items():
    bc = np.array(g.visual.material.baseColorFactor, float)
    bc = bc / 255 if bc.max() > 1.0 else bc
    items.append((name, g, bc))
    tottri += len(g.faces)
    print(f"  {name:18s} tris={len(g.faces):6d} verts={len(g.vertices):6d} "
          f"rgba=({bc[0]:.2f},{bc[1]:.2f},{bc[2]:.2f},{bc[3]:.2f}) "
          f"watertight={g.is_watertight}")
print("total triangles:", tottri)
print("scene bounds:", np.round(scene.bounds, 2).tolist())

# front-view render (glass first, lead proud on top)
items.sort(key=lambda it: 0 if it[2][3] < 0.99 else 1)
fig, ax = plt.subplots(figsize=(4, 6))
ax.set_facecolor((0.11, 0.11, 0.13))
for name, g, bc in items:
    V = g.vertices[:, :2]
    polys = V[g.faces]
    ax.add_collection(PolyCollection(polys, facecolors=[bc[:3]], edgecolors="none"))
ax.set_xlim(0, 200); ax.set_ylim(0, 300); ax.set_aspect("equal"); ax.axis("off")
fig.savefig(d / "rust_front.png", dpi=110, bbox_inches="tight", facecolor=(0.11, 0.11, 0.13))
print("wrote rust_front.png")
