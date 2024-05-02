#include <ftxui/dom/elements.hpp>
#include <ftxui/screen/screen.hpp>
#include <ftxui/component/component.hpp>
#include <ftxui/component/screen_interactive.hpp>
#include "GitManager.h"

using namespace ftxui;

int main() {
    auto screen = ScreenInteractive::Fullscreen();

    GitManager manager("/home/miguevr/GitHub/"); // Read this from a config? Alias and argv[1]?
    std::vector<std::string> repoList = manager.listRepositories();
    int selected = 0;
    std::string details = manager.getDetails(repoList[selected]);
    std::string keysHistory = "";

    auto menu = Menu(&repoList, &selected);
    auto custom_menu = CatchEvent(menu, [&](Event event) -> bool {
        if (event == Event::ArrowUp) {
            if (!repoList.empty()) {
                int next_selection = (selected - 1 + repoList.size()) % repoList.size();
                details = manager.getDetails(repoList[selected]);
            }
            return false;
        }

        if (event == Event::ArrowDown) {
            if (!repoList.empty()) {
                int next_selection = (selected + 1 + repoList.size()) % repoList.size();
                details = manager.getDetails(repoList[selected]);
            }
            return false;
        }
        if (event == Event::g){
            keysHistory += "g";
            if (keysHistory.size() == 2){
                keysHistory = "";
            }
            return true;
        }
        else if (keysHistory == "g") {
            std::string command = "g" + event.character();
            manager.executeCommand(command, repoList[selected], "Updating MIGUE");
            keysHistory = "";
            return true;
        }

        return false; // Event not handled, proceed to default handling.
    });

    // Renderer for displaying the menu and git details
    auto renderer = Renderer(custom_menu, [&] {
        return hbox({
            vbox({
                hbox({
                    filler(),
                    text("Lista Githubs") | hcenter,
                    filler(),
                }) | border | color(Color::Blue),
                custom_menu->Render() | border | color(Color::Blue) | flex,
            }),
            vbox({
                hbox({
                    filler(),
                    text("Commits diff") | hcenter,
                    filler(),
                }) | border | color(Color::Blue),
                paragraph(details) | border | color(Color::Green) | flex,
            }) | flex_grow,
        }) | flex;
    });

    screen.Loop(renderer);
    return 0;
}
