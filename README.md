# ChaxTUI

This is the Terminal User Interface (TUI) of the Chax Project.

## Build and Run

```bash
git clone https://github.com/Chaxware/ChaxTUI
cd ChaxTUI
git checkout ncurses-rewrite
mkdir build
cd build
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON ..
make
./src/chax
```
