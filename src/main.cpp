#include <ncurses.h>

#include <clocale>
#include <vector>

#include "util/Window.hpp"

void sendMessage(Window* chatWindow, std::string message);

int screenHeight, screenWidth;

int main(int argc, char* argv[]) {
  // Initialize curses mode
  initscr();

  // Setup colors
  use_default_colors();
  start_color();

  // Visual configuration
  setlocale(LC_ALL, "");
  raw();
  noecho();
  curs_set(FALSE);

  // Input configuration
  keypad(stdscr, TRUE);

  refresh();

  // Get current dimensions of terminal screen
  getmaxyx(stdscr, screenHeight, screenWidth);

  Window infoBox((Position){1, 1}, (Size){3, screenWidth - 2});
  infoBox.printText(
      (Position){1, screenWidth / 2 - 2},
      "Chax\t\t\t\t\t\t\t\t\t\tPress Ctrl + Q to exit"
  );

  Window chatWindow(
      (Position){4, 1}, (Size){screenHeight - 9, screenWidth - 2}
  );

  Window messageBox(
      (Position){screenHeight - 5, 1}, (Size){5, screenWidth - 2}
  );
  messageBox.printText((Position){2, 4}, "Write a message...", true);

  std::string message = "";

  int ch;
  while ((ch = getch()) != ('q' & 0x1F)) {  // Quit when pressing Ctrl + Q
    switch (ch) {
      case '\n':  // Pressed Enter
        if (message != "") sendMessage(&chatWindow, message);
        message = "";
        break;
      case KEY_BACKSPACE:
        if (!message.empty()) message.pop_back();
        break;
      default:
        message.push_back(ch);
    }

    if (message != "") {
      messageBox.printText((Position){2, 4}, message);
    } else {
      messageBox.printText((Position){2, 4}, "Write a message...", true);
    }
  }

  // Closes curses mode
  endwin();

  return 0;
}

std::vector<std::string> messages;
void sendMessage(Window* chatWindow, std::string message) {
  WINDOW* window = chatWindow->win;
  wclear(window);
  box(window, 0, 0);
  messages.push_back(message);

  for (int i = 0; i < messages.size(); i++) {
    int row = chatWindow->size.height - (messages.size() - i) * 2 - 1;
    if (row <= 1) continue;

    wattron(window, A_BOLD);
    mvwaddstr(window, row, 4, "You: ");
    wattroff(window, A_BOLD);
    waddstr(window, messages.at(i).c_str());
  }
  wrefresh(window);
}
