cmake_minimum_required(VERSION 3.22)

project(test)

include_directories(
    ${PROJECT_SOURCE_DIR}/src
    ${PROJECT_SOURCE_DIR}/include
)

add_executable(test src/main.cpp)