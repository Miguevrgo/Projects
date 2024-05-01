//
// Created by miguevr on 5/1/24.
//

#include "GitManager.h"

std::vector<std::string> GitManager::listRepositories() {
    std::vector<std::string> repositories;
    try {
        for (const auto& entry : std::filesystem::directory_iterator(baseDirectory)) {
            if (entry.is_directory()) {
                auto gitDir = entry.path() / ".git";
                if (std::filesystem::exists(gitDir) && std::filesystem::is_directory(gitDir)) {
                    repositories.emplace_back(entry.path().filename().string());
                }
            }
        }
    }
    catch(const std::filesystem::filesystem_error& err){
        std::cerr << "Error accesing directory: " << err.what() << "\n Revise privileges in base Folder" << std::endl;
    }
    return repositories;
}
