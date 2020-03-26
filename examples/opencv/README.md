# Opencv example
Opencv version used:  3.4.8_3

This example is modified example from opencv [website](https://docs.opencv.org/3.4.8/db/df6/tutorial_erosion_dilatation.html).

Opencv also has morphology on run-length encoded images but it is not used in this example.

## Compilation
Opencv library needed, installation instructions on [website](https://docs.opencv.org/master/df/d65/tutorial_table_of_content_introduction.html).

After installing opencv library, run in shell:
```bash
make
```

## Running program
Program takes one required parameter, which is path to image and one optional
which is morphoolgy operation, dilate or erode, default is dilate.
```bash
./example /path/to/image.png [dilate|erode]
```
