//
// Created by miguevr on 5/1/24.
//

#include "GitManager.h"

std::vector<std::string> GitManager::listRepositories() const {
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

std::string GitManager::gitShortcut(const std::string& shortcut) const {
    // Define the shortcuts for the commands //GIT LOG IMPORTANT BUT NOT HERE
    std::unordered_map<std::string, std::string> shortcuts = {
            {"gs", "status"},
            {"gk", "push"},
            {"gj", "pull"}
            //{"fetch", "fetch"},
    };

    return shortcuts[shortcut];

}

void GitManager::gitCommand(const std::string &command, const std::string &repository) const {
    std::string cmd = "cd " + baseDirectory + "/" + repository + "&& git " + command;
    std::string outputRedirect = cmd + " &> /dev/null";
    std::system(outputRedirect.c_str());
}

void GitManager::executeCommand(const std::string &command, const std::string& repository,
        const std::string commit) const 
{
    std::string toExecute = gitShortcut(command);
    if (commit == "NOT_PUSH") {
        gitCommand(toExecute, repository);
    }
    else{
        gitCommand("add .", repository);
        gitCommand("commit -m \"" + commit + "\"", repository);
        gitCommand(toExecute,repository);
    }
}