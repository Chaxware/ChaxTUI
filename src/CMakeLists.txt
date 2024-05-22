# Ensure relative paths are converted to absolute paths
cmake_policy(SET CMP0076 NEW)

set(SOURCES
  main.cpp

  util/Window.cpp
)

add_executable(chax ${SOURCES})

target_include_directories(chax PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../include)

target_link_libraries(chax ${NCURSES_LIBRARIES} panel)
