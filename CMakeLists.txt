cmake_minimum_required(VERSION 3.16)

project(sacrifice VERSION 1.0 LANGUAGES CXX)

add_library(${PROJECT_NAME} cxx/sac.h cxx/sac.cpp)

include(FetchContent)
FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG master
)
FetchContent_MakeAvailable(Corrosion)

find_program(CXXBRIDGE cxxbridge PATHS "$ENV{HOME}/.cargo/bin/")
if (CXXBRIDGE STREQUAL "CXXBRIDGE-NOTFOUND")
    message("Could not find cxxbridge, trying to install with `cargo install cxxbridge-cmd'")
    find_program(CARGO cargo PATHS "$ENV{HOME}/.cargo/bin/")
    if (CARGO STREQUAL "CARGO-NOTFOUND")
        message(FATAL_ERROR "Requires cargo available in path, install via rustup https://rustup.rs/")
    endif()
    execute_process(COMMAND ${CARGO} install cxxbridge-cmd)
    find_program(CXXBRIDGE cxxbridge PATHS "$ENV{HOME}/.cargo/bin/")
endif()

corrosion_import_crate(MANIFEST_PATH ffi/Cargo.toml)
corrosion_add_cxxbridge(
        librustsacrifice
        CRATE sacrifice-ffi
        MANIFEST_PATH ffi
        FILES lib.rs
)

target_link_libraries(${PROJECT_NAME} PRIVATE librustsacrifice)

add_executable(${PROJECT_NAME}-example cxx/bin/example.cpp)
target_link_libraries(${PROJECT_NAME}-example PRIVATE ${PROJECT_NAME})
