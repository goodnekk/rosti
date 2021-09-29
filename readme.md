# Rosti
A crispy software rasteriser written in rust

## Goals
- Draw filled polygons and strokes
- Platform independent
- Fast (at least for a software rasterizer)
- No bloat & minimal dependencies
- Antialiassing
- Well documented API

## Non Goals
- Reading vector files
- Reading font files
- Drawing to screen
- Writing to image files
- GPU acceleration

## Todo

#### Path rendering
- [x] Naive line
- [ ] Basic Bresenham line
- [ ] Antialiased line (Xiaolin Wu)

#### Geometry
- [x] Quadratic & Cubic Beziers
- [ ] Generate stroke width geometry
- [ ] Line Caps
- [ ] Line Join (Miter, Bevel, Round)
- [ ] Polynomials & Splines
- [ ] Circles

#### Fills
- [x] Basic Polygon fills
- [ ] Antialiased Polygon fills
- [ ] Basic rectangle
- [ ] Basic circle
- [ ] Antialiassed circle