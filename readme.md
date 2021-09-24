# Rosti
A crispy software rasteriser written in rust

## Goals
- Draw filled polygons and strokes
- Platform independent
- Fast
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

#### Strokes
- [x] Naive line
- [ ] Basic Bresenham line
- [ ] Antialiased line (Xiaolin Wu)
- [ ] Quadratic & Cubic Beziers
- [ ] Polynomials & Splines
- [ ] Circles
- [ ] Line Caps
- [ ] Line Join (Miter, Bevel, Round)

#### Fills
- [ ] Basic Polygon fills
- [ ] Antialiased Polygon fills
- [ ] Basic rectangle
- [ ] Basic circle
- [ ] Antialiassed circle