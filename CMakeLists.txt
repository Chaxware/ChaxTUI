cmake_minimum_required(VERSION 3.5)
project(chax LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

find_package(Curses REQUIRED)
include_directories(${CURSES_INCLUDE_DIR})

# add_executable(chax src/main.cpp)

add_subdirectory(src)

# target_link_libraries(chax ${CURSES_LIBRARIES})
