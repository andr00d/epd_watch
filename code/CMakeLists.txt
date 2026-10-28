cmake_minimum_required(VERSION 3.20.0)
find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})

project(watch)

FILE(GLOB app_sources include/*.h)
target_include_directories(app PRIVATE include)

target_sources(app PRIVATE 
        src/main.cpp
        src/display.cpp
        src/display_font.cpp
        src/display_shapes.cpp
        src/io.cpp
        src/pages.cpp
)
