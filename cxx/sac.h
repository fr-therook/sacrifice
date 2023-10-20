//
// Created by soy on 10/18/23.
//

#ifndef SACRIFICE_SAC_H
#define SACRIFICE_SAC_H

#include <string>

namespace sac {
class Square {
private:
    unsigned int m_file;
    unsigned int m_rank;

public:
    Square(unsigned int file, unsigned int rank)
        : m_file(file), m_rank(rank) {}

    unsigned int file() const { return m_file; }
    unsigned int rank() const { return m_rank; }
};

class Game {
private:
    class Impl;
    Impl* impl;

public:
    struct Node {
        class Impl;
        Impl* impl;
    };

    Game();
    ~Game();

    explicit Game(std::string pgn_str);



    std::string to_pgn();
};
}

#endif //SACRIFICE_SAC_H
