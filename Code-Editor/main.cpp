#include <iostream>

int main() {
    char input;
    while(true) {
        std::cin >> input;
        if (input != ':'){
            std::cout << input;
        }
        else{
            std::cin >> input;
            switch(input){
                case 'q':
                    break;
                case 's':
                    // Save current terminal content to file with name by buffer
                    break;
            }
        }
    }
    return 0;
}
