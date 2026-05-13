cmake_minimum_required(VERSION 3.10)
project(loopdynamo)

set(CMAKE_CXX_STANDARD 20)

add_library(loopdynamo SHARED
    src/node.cpp
    src/runner.cpp
    src/flow.cpp
    src/print_node.cpp
    src/api.cpp
)
