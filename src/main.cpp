#include <ncurses.h>

#include <clocale>

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

  WINDOW *greetingWindow =
      newwin(5, 30, screenHeight / 2 - 3, screenWidth / 2 - 15);
  box(greetingWindow, 0, 0);  // Draw border
  mvwprintw(greetingWindow, 0, 6, " Hello Internet! ");
  mvwprintw(greetingWindow, 2, 7, "Welcome to Chax");
  wrefresh(greetingWindow);

  getch();  // Get character input

  // Closes curses mode
  endwin();

  return 0;
}
