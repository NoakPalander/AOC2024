cmake_minimum_required(VERSION 3.30)

project(cpp)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(cpp main.cpp)
target_compile_options(cpp PRIVATE
  -std=c++26
  -Wall
  -Wextra
  -Wconversion
  -pedantic-errors
  -O3)
  -ggdb3
  -fno-omit-frame-pointer
  -fsanitize=address,undefined)

target_link_options(cpp PRIVATE -fsanitize=address,undefined)
