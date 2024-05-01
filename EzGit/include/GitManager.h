//
// Created by miguevr on 5/1/24.
//

#ifndef EZGIT_GITMANAGER_H
#define EZGIT_GITMANAGER_H

#include <string>
#include <utility>
#include <vector>
#include <cstdlib>
#include <cstdio>
#include <array>
#include <iostream>
#include <unordered_map>
#include <filesystem>

class GitManager {
public:
    explicit GitManager(std::string  baseDir) : baseDirectory(std::move(baseDir)){}
    //TODO: ELiminar también 
    std::string exec(const char* cmd) {
        std::array<char, 128> buffer;
        std::string result;
        std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
        if (!pipe) {
            throw std::runtime_error("popen() failed!");
        }
        while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
            result += buffer.data();
        }
        return result;
    }

    /**
     * @brief TEST FUNCTION FOR GIT LOG //TODO
     * 
     *
     */
    std::string getDetails(const std::string& repo) {
        // Construct the command to change to the repository's directory and execute git status
        std::string cmd = "cd '" + baseDirectory + "/" + repo + "' && git status";
        // Use exec to run the command and return the output
        return exec(cmd.c_str());
    }



    /**
     * This functions list all the found repositories within the base
     * Directory, it returns a vector of strings containing the name of
     * the repositories found
     *
     * @note / is overloaded in filesystem for concatenation of paths
     *
     * @return repositories found
     */
    std::vector<std::string> listRepositories() const;

    /**
     * @brief This function will return the actual git command for a given
     * shortcut, this is used to translate the shortcuts used by the application
     * to the actual git command
     * 
     * @param shortcut shortcut to translate
     * @return string git command
     */
    std::string gitShortcut(const std::string& shortcut) const;

    /**
     * @brief This function will execute a git command in the repository,
     * it executes the exact command passed as a string, shortcuts used by
     * the application are translated to the actual git command from gitShortcut
     * function
     * 
     * @param command git command to execute: git <command> 
     * @param repository repository to execute the command in
     */
    void gitCommand(const std::string &command, const std::string &repository) const;

    /**
     * @brief This function receives a command in shortcut form and executes it
     * in the provided repository using the gitCommand function along with the
     * gitShortcut function to translate the command
     * 
     * @param command command to execute in shortcut form
     */
    void executeCommand(const std::string &command, const std::string& repository,
        const std::string commit = "NOT_PUSH") const;

private:
    std::string baseDirectory;
};


#endif //EZGIT_GITMANAGER_H
