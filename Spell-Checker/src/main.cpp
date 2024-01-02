#include <iostream>
#include "Corrector.h"

int main(int argc, char* argv[]) {

    std::string resourcedir = "./resources";

    // Loading word from input
    int option;
    do {
        std::cout << "Choose a language: " << std::endl;
        std::cout << "\t [1] English" << std::endl;
        std::cout << "\t [2] Spanish" << std::endl;
        std::cout << "Option: ";
        std::cin >> option;
    } while (option != 1 && option != 2);

    std::string language = (option == 1) ? "english.txt" : "spanish.txt";

    Dictionary dictionary(resourcedir + "/" + language, language);
    Corrector corrector(dictionary);

    std::string word;
    do{
        std::cout << "Enter a word, or type 'exit' to exit: ";
        std::cin >> word;

        if (word == "exit"){break;}

        std::vector<std::string> suggestions = corrector.GetTopSuggestions(corrector.SuggestCorrections(word), 5);
        for (const auto& suggestion : suggestions){
            std::cout << suggestion << std::endl;
        }
    } while (word != "exit");

    return 0;
}
