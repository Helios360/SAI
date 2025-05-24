cmake_minimum_required(VERSION 3.10)
project(MyProject)

find_package(OpenCV REQUIRED)
find_package(OpenCV REQUIRED COMPONENTS face)

add_executable(MyExecutable main.cpp)
target_link_libraries(MyExecutable ${OpenCV_LIBS})
include_directories(${OpenCV_INCLUDE_DIRS})
