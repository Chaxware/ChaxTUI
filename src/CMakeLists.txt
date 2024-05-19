# Ensure relative paths are converted to absolute paths
cmake_policy(SET CMP0076 NEW)

set(SOURCES
  main.cpp
)

add_executable(chax ${SOURCES})

target_link_libraries(chax ${NCURSES_LIBRARIES})

target_include_directories(chax PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../include)
