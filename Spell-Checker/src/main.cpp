#include <iostream>
#include "Corrector.h"

int main(int argc, char* argv[]) {
    if (argc != 2) {
        std::cout << "Usage: " << argv[0] << " <resource>" << std::endl;
        return 1;
    }

    std::string resourcedir = argv[1];
    Dictionary spanishTest(resourcedir + "/spanish.txt", "es");
    Dictionary englishTest(resourcedir + "/english.txt", "en");
    Corrector correctores(spanishTest);
    Corrector correctoren(englishTest);

    std::string word = "hellp";
    std::cout << "Suggesting corrections for " << word << " in English:" << std::endl;
    std::vector<std::string> topSuggestions = correctoren.GetTopSuggestions(correctoren.SuggestCorrections(word), 5);
    for (const auto& suggestion : topSuggestions){
        std::cout << suggestion << std::endl;
    }
    return 0;
}
