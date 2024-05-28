#include "iostream"

const int a = 10;
/*
* calculate factorial of A
* print it only if it's even
*/
int main() {

  int factorial = 1;
  // factorial of 10 = 10*9*8...*1
  int i = 0;
  do {
    factorial = factorial * i;
    i += 1;
  } while (i <= a);

  if (factorial % 2 == 0) {
    printf("%d", factorial);
  } else {
    printf("nope!");
  }

  return 0;
}
