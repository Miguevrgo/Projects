# Spell Checker

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

Just clone the repository and compile with the given CmakeList file, you can easily explore the different files as there are not many and the name is quite descriptive. For the language files, just consider that it'll work as long as
you provide a valid file with a word per column with its frequency associated.

## Contributing

I welcome contributions from the community. Whether it's improving the algorithm, adding support for new languages, or fixing bugs, your input is valuable.

## License

This project is licensed under the same license as the repository: GNU AFFERO GENERAL PUBLIC LICENSE v3.0

