## PPM

The PPM (Portable Pixmap) format is a simple, uncompressed color image format, often considered the "lowest common denominator" for color images.

While easy to read and write—making it ideal for educational or experimental use—the format is extremely inefficient. It contains a lot of redundant data and often includes more information than the human eye can perceive. Additionally, it stores only basic pixel color data (typically red, green, and blue values) with no metadata, meaning any meaningful use may require pairing it with external data.

In practice, many files are labeled as PPM despite deviating from the standard—particularly in how color values are interpreted. These variants are commonly accepted due to the PPM format's role as an intermediary in processing pipelines. To be precise, it's best to describe such files with their specific variations, e.g., "PPM using the RGB color space of a particular scanner."



## 🟠 Midpoint Circle Algorithm

The **Midpoint Circle Algorithm** is a fast, integer-based method to draw circles in raster graphics. It calculates points in one octant and mirrors them to complete the circle, using only addition and subtraction (no floating point math).

### 🔧 How It Works

- Start at top: `(x = 0, y = r)`
- Initial decision parameter: `p = 1 - r`
- For each step:
  - If `p < 0`:  
    `x += 1`, `p += 2x + 1`
  - Else:  
    `x += 1`, `y -= 1`, `p += 2x - 2y + 1`
- Plot 8 symmetric points for each `(x, y)`.

### ➗ Symmetry (8-way)

Use symmetry to draw all octants from one point:
```
(x, y), (y, x), (-x, y), (-y, x),
(-x, -y), (-y, -x), (x, -y), (y, -x)
```

### References:
* [PPM - Netpbm Documentation](https://netpbm.sourceforge.net/doc/ppm.html)
