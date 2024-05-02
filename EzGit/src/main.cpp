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

    auto menu = Menu(&repoList, &selected);

    auto renderer = Renderer(menu, [&] {
        return hbox({
            vbox({
                hbox({
                    filler(),
                    text("Lista Githubs") | hcenter,
                    filler(),
                }) | border | color(Color::Blue),

                menu->Render() | border | color(Color::Blue) | flex,
            }),

            vbox({
                hbox({
                    filler(),
                    text("Commits diff") | hcenter,
                    filler(),
                }) | border | color(Color::Blue),

                paragraph(manager.getDetails(repoList[selected])) | border | color(Color::Green) | flex,

            }) | flex_grow,
        }) | flex;
    });

    screen.Loop(renderer);
    return 0;
}
