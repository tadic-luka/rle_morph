# RLE morph
Library which uses modified run-length encoding to do basic morphological operations on binary images (dilation and erosion).

Algorithm is taken from paper [Fast Algorithms for Binary Dilation and Erosion Using Run-Length Encoding ](https://pdfs.semanticscholar.org/0535/5c17fe35eedfd4b6055a010fd65c5a25f04a.pdf)

Features:
- [x] Dilation
- [ ] Erosion
- [ ] Optimize implementation


## Some basic performance comparison:
Intel i7-8750H, 6 cores, linux 5.4.23_1

| Library | Dilate | Erode |
| --- | --- | --- |
| rle_morph |  ~4600 us | not implemented |
| [opencv example](examples/opencv) |  ~12600 us | ~12600 us|

Details about opencv used are explained in examples/opencv/README.md.
