#include "opencv2/imgproc.hpp"
#include "opencv2/highgui.hpp"
#include <iostream>
#include <chrono>
using namespace cv;
using namespace std;
using namespace std::chrono;

const String keys = 
"{@input | <none> | input image}"
"{@morph ? | dilate | morph operation (erode|dilate)}"
;

const int morph_size = 5;
Mat Dilation(Mat);
Mat Erosion(Mat);

void printUsage(char *prog_name)
{
	cout << "Usage: " << prog_name << " <Input image> [erode|dilate]" << endl;
}

int main( int argc, char** argv )
{
	CommandLineParser parser( argc, argv, keys);
	if (!parser.check()) {
		parser.printErrors();
		return 1;
	}
	String morph_type = parser.get<String>("@morph");
	Mat src = imread( samples::findFile( parser.get<String>( "@input" ) ), IMREAD_COLOR );
	if( src.empty() )
	{
		cout << "Could not open or find the image!\n" << endl;
		printUsage(argv[0]);
		return 1;
	}
	Mat dst;
	if (morph_type == "dilate") {
		dst =  Dilation(src);
	} else if ( morph_type == "erode" ) {
		dst = Erosion(src);

	} else {
		cout << "Invalid morph operation!\n" << endl;
		printUsage(argv[0]);
		return 1;
	}
	imwrite("output.png", dst);
	return 0;
}

Mat Dilation(Mat src)
{
	Mat dst;
	Mat element = getStructuringElement( MORPH_RECT,
			Size( 2*morph_size + 1, 2*morph_size+1 ),
			Point( morph_size, morph_size ) );
	auto start = high_resolution_clock::now();
	dilate( src, dst, element );
	auto stop = high_resolution_clock::now();
	auto duration = duration_cast<microseconds>(stop - start);
	cout << "Time taken to dilate: " << duration.count() << " microseconds" << endl;
	return dst;
}

Mat Erosion(Mat src)
{
	Mat dst;
	Mat element = getStructuringElement( MORPH_RECT,
			Size( 2*morph_size + 1, 2*morph_size+1 ),
			Point( morph_size, morph_size ) );
	auto start = high_resolution_clock::now();
	erode( src, dst, element );
	auto stop = high_resolution_clock::now();
	auto duration = duration_cast<microseconds>(stop - start);
	cout << "Time taken to erode: " << duration.count() << " microseconds" << endl;
	return dst;
}
