//
// Created by miguevr on 16/10/23.
//

#ifndef IMAGENRGB_IMAGE_H
#define IMAGENRGB_IMAGE_H

typedef unsigned char byte;

struct Pixel {
    byte r, g, b;
    Pixel(byte red, byte green, byte blue) : r(red) , g(green), b(blue) {}
};

class Image {
public:
    Image();
    Image(int nrow,int ncol,Pixel pixel= Pixel(0,0,0));
    int GetRows();
    int GetCols();
private:
    int rows;
    int cols;
    Pixel** image;
};


#endif //IMAGENRGB_IMAGE_H
