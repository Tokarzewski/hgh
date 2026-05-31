import json, sys
from pathlib import Path
import numpy as np
from PIL import Image

src = Path(r"C:\Users\model\Documents\Github\hgh\assets\image library\TEST_IMAGE.jpg")
im = Image.open(src).convert("RGBA")
arr = np.asarray(im)
out = Path(__file__).parent
arr.tofile(out / "img.rgba")
json.dump({"w": im.width, "h": im.height, "ch": 4}, open(out / "img.json", "w"))
print("dumped", im.width, "x", im.height, "RGBA ->", (out / "img.rgba"))
