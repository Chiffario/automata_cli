#include "stdio.h"

int get_digit_sum(int number) {
    return (number % 10) + (number / 10);
}

int find_result(int crab, int fish) {
    printf("Введите число Жоры и число щуки\n");
    int fish;
    int crab;

    // Числа Жоры и щуки должны быть в пределах от 10 до 19
    if (crab > 19 || crab < 10) {
        printf("Число Жоры вне допустимых пределов");
        return 0;
    }
    if (fish > 19 || fish < 10) {
        printf("Число щуки вне допустимых пределов");
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
        printf("Жора: %i, Щука: %i, Щука выиграла\n", crab, fish);
        return 0;
    }
    if (fish < 100 && crab >= 100) {
        printf("Жора: %i, Щука: %i, Жора выиграл\n", crab, fish);
        return 0;
    }
    if (fish >= 100 && crab >= 100) {
        printf("Жора: %i, Щука: %i, Ничья!\n", crab, fish);
        return 0;
    }
    return 0;
}

/*
 * run it through to check all the possible values
 */

int main() {
    for (int i = 10; i < 20; i++) {
        for (int j = 10; j < 20; j++) {
            printf("crab: %i, fish: %i => ", i, j);
            find_result(i, j);
        }
    }
}