#include "stdio.h"
int get_digit_sum(int number) {
    return (number % 10) + (number / 10);
}

int find_result(int crab, int fish) {

    // Числа Жоры и щуки должны быть в пределах от 10 до 19
    if (crab > 19 || crab < 10) {
        return 0;
    }
    if (fish > 19 || fish < 10) {
        return 0;
    }

    // Пока один из них не выиграл
    while ((fish < 100) && (crab < 100)) {
        // Считаем суммы цифр
        int fish_digits = get_digit_sum(fish);
        int crab_digits = get_digit_sum(crab);
        // Прибавляем к исходным значениям суммы цифр
        crab += fish_digits;
        fish += crab_digits;
//        printf("Жора: %i(+%i), щука: %i(+%i)\n", crab, fish_digits, fish, crab_digits);
    }
    if (fish >= 100 && crab < 100) {
        return 0;
    }
    if (fish < 100 && crab >= 100) {
        return 0;
    }
    if (fish >= 100 && crab >= 100) {
        return 0;
    }
    return 0;
}

/*
 * run it through to check all the possible values
 */

int main() {

}
