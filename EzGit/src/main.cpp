#include <ftxui/dom/elements.hpp>
#include <ftxui/screen/screen.hpp>
#include <ftxui/component/component.hpp>
#include <ftxui/component/screen_interactive.hpp>
#include "GitManager.h"

using namespace ftxui;

int main() {
    auto screen = ScreenInteractive::Fullscreen();

    // Repository list and selected index
    std::vector<std::string> repoList = {"ALGO", "Projects", "FP"};
    int selected = 0;

    // Menu component
    auto menu = Menu(&repoList, &selected);

    // Using a Renderer to customize the display of the menu and paragraphs
    auto renderer = Renderer(menu, [&] {
        return hbox({
            vbox({
                 paragraph("Lista Githubs") | border | color(Color::Blue),
                 menu->Render() | border | color(Color::Blue) | flex,
            }),

            vbox({
                hbox({
                    filler(),
                    text("Commits diff") | hcenter,
                    filler(),
                }) | border | color(Color::Blue),

                paragraph("Esto representa los cambios") | border | color(Color::Green) | flex,

            }) | flex_grow,
        }) | flex;
    });

    screen.Loop(renderer);
    return 0;
}
