//
// Created by soy on 10/18/23.
//

#include "sac.h"
#include "librustsacrifice/lib.h"

class sac::Game::Impl {
private:
    friend class sac::Game;

    rust::Box<rustsac::GameTree> tree;
public:
    Impl() : tree(rustsac::game_default()) {}
    explicit Impl(std::string pgn_str)
        : Impl() {
        auto* tree_ptr = rustsac::game_from_pgn({pgn_str});
        if (tree_ptr == nullptr) return;

        tree = rust::Box<rustsac::GameTree>::from_raw(tree_ptr);
    }
};

sac::Game::Game() : impl(new Impl()) {}
sac::Game::~Game() { delete impl; }

sac::Game::Game(std::string pgn_str) : impl(new Impl(pgn_str)) {}

std::string sac::Game::to_pgn() {
    return impl->tree->pgn().operator std::string();
}
