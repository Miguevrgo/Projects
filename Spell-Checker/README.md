# Spell Checker
<p align="center">
  <img src="https://raw.githubusercontent.com/Miguevrgo/Projects/master/Spell-Checker/Spell.gif" alt="Spell Checker" />
</p>

## Objective

The aim of this project is to develop a modern C++ program capable of correcting and suggesting the most probable words for given inputs. It's designed to enhance text accuracy and readability by providing intelligent spelling corrections.

## Features

- **Language Selection**: Users can choose the language for spell checking, allowing for a more customized and relevant correction experience based on the linguistic context.
- **Dictionary Data Structure**: The program employs a specialized Dictionary data structure to load and manage language data efficiently.
- **Advanced Correction Algorithm**: Utilizes a blend of the Levenshtein algorithm and a custom algorithm that measures the distance between mistyped keys based on a QWERTY ANSI keyboard layout. This approach ensures high accuracy in suggesting corrections by considering the physical layout of widely used keyboards.
- **Word Frequency Analysis**: Incorporates a learning component that assigns frequencies to words based on their occurrence in extensive language corpora, enabling the program to suggest more likely corrections based on word usage patterns.

## Implementation Details

The spell checker is built with a focus on performance and accuracy. By taking into account the nuances of keyboard layouts and language-specific word frequencies, it offers a sophisticated solution to common spelling errors. This project is ideal for anyone looking to enhance text quality in applications ranging from text editors to user input validation systems.

## Usage

To use it, just clone the repository, go to the project folder, create a build directory, and build it using cmake. You have to install Qt if not already.
### Debian
```
sudo apt-get install qtbase5-dev qtchooser qt5-qmake qtbase5-dev-tools
git clone https://github.com/Miguevrgo/Projects.git 
cd Projects/Spell-Checker
mkdir build
cd build
cmake ..
make
```


## Contributing

I welcome contributions from the community. Whether it's improving the algorithm, adding support for new languages, or fixing bugs, your input is valuable.

## License

This project is licensed under the same license as the repository: GNU AFFERO GENERAL PUBLIC LICENSE v3.0

