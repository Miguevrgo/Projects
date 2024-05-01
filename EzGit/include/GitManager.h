//
// Created by miguevr on 5/1/24.
//

#ifndef EZGIT_GITMANAGER_H
#define EZGIT_GITMANAGER_H

#include <string>
#include <utility>
#include <vector>
#include <cstdlib>
#include <iostream>
#include <filesystem>

class GitManager {
public:
    explicit GitManager(std::string  baseDir) : baseDirectory(std::move(baseDir)){}
    /**
     * This functions list all the found repositories within the base
     * Directory, it returns a vector of strings containing the name of
     * the repositories found
     *
     * @note / is overloaded in filesystem for concatenation of paths
     *
     * @return repositories found
     */
    std::vector<std::string> listRepositories();
private:
    std::string baseDirectory;
};


#endif //EZGIT_GITMANAGER_H
