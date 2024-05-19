#include "util/Window.hpp"

Window::Window(Position position, Size size) : position(position), size(size) {
  win = newwin(size.height, size.width, position.y, position.x);
  panel = new_panel(win);

  box(win, 0, 0);
  wrefresh(win);
}

Window::~Window() { delwin(win); }
