#include "util/Window.hpp"

Window::Window(Position position, Size size) : position(position), size(size) {
  win = newwin(size.height, size.width, position.y, position.x);
  panel = new_panel(win);

  box(win, 0, 0);
  wrefresh(win);
}

void Window::printText(
    Position textPosition, std::string text, bool mutedColor
) {
  wclear(win);
  box(win, 0, 0);

  if (mutedColor) {
    wattron(win, COLOR_PAIR(1));
    init_pair(1, COLOR_WHITE, -1);
  }

  mvwprintw(win, textPosition.y, textPosition.x, "%s", text.c_str());

  if (mutedColor) {
    wattroff(win, COLOR_PAIR(1));
  }

  wrefresh(win);
}

Window::~Window() { delwin(win); }
