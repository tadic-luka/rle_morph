# RLE morph
Library which uses modified run-length encoding to do basic morphological operations on binary images (dilation and erosion).

Algorithm is taken from paper [Fast Algorithms for Binary Dilation and Erosion Using Run-Length Encoding ](https://pdfs.semanticscholar.org/0535/5c17fe35eedfd4b6055a010fd65c5a25f04a.pdf)

Features:
- [x] Dilation
- [x] Erosion (implemented but with custom algorithm instead with algorithm in paper)
- [ ] Optimize implementation


## Some basic performance comparison (by running examples):
Intel i7-8750H, 6 cores, linux 5.4.23_1

| Library | Dilate | Erode |
| --- | --- | --- |
| rle_morph |  ~4600 us | ~6800 us |
| [opencv example](examples/opencv) |  ~12600 us | ~12600 us|
| imgproc | ~71800 us | ~47000 us |

rle_morph and imgproc are compiled in release mode.

imgproc version used: 0.20.0

image version used: 0.23.2

Details about opencv used are explained in examples/opencv/README.md.
