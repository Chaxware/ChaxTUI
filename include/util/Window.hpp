#pragma once

#include <ncurses.h>
#include <panel.h>

#include <string>

struct Position {
  int y;
  int x;
};

struct Size {
  int height;
  int width;
};

class Window {
 public:
  WINDOW* win;
  PANEL* panel;

  Position position;
  Size size;

  Window(Position position, Size size);

  void printText(
      Position textPosition, std::string text, bool mutedColor = false
  );

  ~Window();
};
