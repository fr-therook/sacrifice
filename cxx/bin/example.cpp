//
// Created by soy on 10/18/23.
//

#include <iostream>

#include "../sac.h"

const char* PGN_STR_1 = "[Event \"Casual Rapid game\"]\n"
                        "[Site \"https://lichess.org/sMqT4A1f\"]\n"
                        "[Date \"2023.10.16\"]\n"
                        "[White \"soyflourbread\"]\n"
                        "[Black \"maia1\"]\n"
                        "[Result \"1-0\"]\n"
                        "[UTCDate \"2023.10.16\"]\n"
                        "[UTCTime \"05:16:08\"]\n"
                        "[WhiteElo \"1500\"]\n"
                        "[BlackElo \"1496\"]\n"
                        "[BlackTitle \"BOT\"]\n"
                        "[Variant \"Standard\"]\n"
                        "[TimeControl \"600+0\"]\n"
                        "[ECO \"D00\"]\n"
                        "[Opening \"Queen's Pawn Game: Accelerated London System\"]\n"
                        "[Termination \"Normal\"]\n"
                        "[Annotator \"lichess.org\"]\n"
                        "\n"
                        "1. d4 d5 2. Bf4 { D00 Queen's Pawn Game: Accelerated London System } Nc6?! { (0.00 → 0.59) Inaccuracy. c5 was best. } (2... c5 3. e3 Nf6 4. Nf3 Nc6 5. Nbd2 Qb6 6. dxc5 Qxb2 7. Rb1) 3. e3 e6 4. Nf3 Bd6 5. Ne5?! { (0.64 → 0.00) Inaccuracy. Bg3 was best. } (5. Bg3 Nf6 6. c4 O-O 7. Nbd2 Re8 8. Ne5 Ne7 9. Bd3 c5) 5... Nxe5 6. dxe5 Bb4+ 7. c3 Bc5?! { (0.79 → 1.54) Inaccuracy. Ba5 was best. } (7... Ba5 8. Nd2 Ne7 9. Bd3 c5 10. O-O Bd7 11. Bc2 Bc7 12. c4) 8. Bd3 Ne7 9. Nd2 O-O 10. O-O?! { (1.59 → 0.89) Inaccuracy. Qh5 was best. } (10. Qh5 Ng6 11. Nf3 Bd7 12. h4 Be8 13. Bg3 f6 14. exf6 gxf6 15. Qh6 Rf7 16. h5 Ne7) 10... Nf5?! { (0.89 → 1.53) Inaccuracy. Ng6 was best. } (10... Ng6 11. Qh5 h6 12. Bg3 Qg5 13. Qd1 Bd7 14. Kh1 Qe7 15. Rc1 a5 16. Qg4 Be8 17. Qe2) 11. b4?! { (1.53 → 0.78) Inaccuracy. Qg4 was best. } (11. Qg4 f6 12. Nf3 fxe5 13. Bxe5 Qe7 14. Rae1 a5 15. e4 Nd6 16. Qg3 dxe4 17. Bxe4 Nf5) 11... Bb6?! { (0.78 → 1.46) Inaccuracy. Be7 was best. } (11... Be7) 12. a4 a5 13. Nb3?! { (1.50 → 0.80) Inaccuracy. Qg4 was best. } (13. Qg4 c6 14. Nf3 axb4 15. cxb4 f6 16. exf6 Qxf6 17. Be5 Qe7 18. e4 Nd6 19. Qg3 Bc7) 13... axb4 14. a5 bxc3?? { (1.12 → 3.48) Blunder. Ba7 was best. } (14... Ba7 15. cxb4) 15. axb6 Rxa1 16. Qxa1 cxb6 17. Qxc3 Bd7 18. Nd4 Nxd4 19. exd4 b5 20. Rb1 Qb6 21. Qb4 Rc8 22. Bxb5?! { (4.17 → 3.18) Inaccuracy. Bd2 was best. } (22. Bd2 Ra8 23. Qb2 Ra4 24. Bb4 Qa6 25. Qc3 Qc6 26. Qd2 h6 27. h3 Qc7 28. h4 Be8) 22... Bxb5 23. Qxb5 Qxd4 24. Qxb7 Rc2?? { (3.42 → Mate in 2) Checkmate is now unavoidable. Rf8 was best. } (24... Rf8 25. Bg3 h6 26. h4 Qc3 27. Kh2 Rc8 28. Qe7 Qc7 29. Qxc7 Rxc7 30. h5 Kh7 31. f3) 25. Qb8+ Rc8 26. Qxc8# { White wins by checkmate. } 1-0";

int main() {
    std::string pgn{PGN_STR_1};

    auto game = sac::Game(pgn);
    std::cout << game.to_pgn() << std::endl;

    return 0;
}
