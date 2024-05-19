#include <ncurses.h>

#include <clocale>

#include "util/Window.hpp"

int screenHeight, screenWidth;

int main(int argc, char *argv[]) {
  // Initialize curses mode
  initscr();

  // Setup colors
  use_default_colors();
  start_color();

  // Visual configuration
  setlocale(LC_ALL, "");
  cbreak();
  noecho();
  curs_set(FALSE);

  // Input configuration
  keypad(stdscr, TRUE);

  refresh();

  // Get current dimensions of terminal screen
  getmaxyx(stdscr, screenHeight, screenWidth);

  Window greetingWindow(
      (Position){screenHeight / 2 - 3, screenWidth / 2 - 15}, (Size){5, 30}
  );
  mvwprintw(greetingWindow.win, 0, 6, " Hello Internet! ");
  mvwprintw(greetingWindow.win, 2, 7, "Welcome to Chax");
  wrefresh(greetingWindow.win);

  getch();  // Get character input

  // Closes curses mode
  endwin();

  return 0;
}
