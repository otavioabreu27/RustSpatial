<img src="https://github.com/otavioabreu27/GeoRust/blob/main/media/icon.png" align="right" width="64"/>

# GeoRust

<!-- badges: start -->
<div style="display: flex; justify-content: center; gap: 5px;">
  <img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
</div>
<!-- badges: end -->

## Summary

Rust implementation of Geographic and Vetorial structures and operations. Focused on memory and thread safety and multithread processing.


## Features

| Feature                 | Description                                                                                                                                                     | Status  |
|-------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------|---------|
| Vertex                  | A primitive representing a single point, also known as a "Vertex".                                                                                              | Done    |
| Line                    | A primitive representing a straight line segment between two points.                                                                                            | Done    |
| Path                    | A primitive representing a sequence of connected lines where the final vertex does not connect back to the starting vertex.                                      | Done    |
| Polygon                 | A primitive representing a closed shape formed by connected lines where the final vertex connects back to the starting vertex.                                   | Pending |
| Geographical Operations | Implements spatial operations such as overlay, intersects, within, touches, crosses, equals, and more.                                                          | Pending |
| Performance Optimization| Optimizes geographical operations using multithreading to maximize scalar type utilization while ensuring thread safety as per the architecture's design.        | Pending |
| Results Presentation    | Evaluates tradeoffs, presents the results of experiments, and publishes comparisons for further study and potential application.                                  | Pending |