#include <iostream>
#include <fstream>
#include <string>
#include <array>

using namespace std;

int main(){
    const int row_size = 2868;
    const int col_size = 4309;
    char* Img[row_size][col_size];

    const int size = row_size * col_size * 3;

    char * memblock; 
    memblock = new char [size];
    
    ifstream file("raw_img.np");
    file.read(memblock, size);

    cout << (int)memblock[21668976];
    
    int c=0;



    /*for(int i=0; i<3; i++){
        for(int j=0; j<col_size;j++){
            cout << Img[i][j][0] << Img[i][j][1] << Img[i][j][2] << endl;

        };
    break;
    };*/
}