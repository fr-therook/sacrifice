//
// Created by soy on 10/18/23.
//

#include <iostream>

#include "sac.h"
#include "librustsacrifice/lib.h"

namespace sac {
    void initialize() {
        auto game = rustsac::game_default();
        std::cout << game->pgn().operator std::string() << std::endl;
    }
}
