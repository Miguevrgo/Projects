#include <iostream>
#include "Corrector.h"

int main(int argc, char* argv[]) {
    if (argc != 2) {
        std::cout << "Usage: " << argv[0] << " <resource>" << std::endl;
        return 1;
    }

    std::string resourcedir = argv[1];
    Dictionary spanishTest(resourcedir + "/spanish.txt", "es");
    Corrector corrector(spanishTest);
    return 0;
}
