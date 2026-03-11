#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

int main(void)
{
  char cards[1024];
  while (fgets(cards, sizeof(cards), stdin), *cards != '0') {
    int ones = 0;
    int score = 0;
    char *card = strtok(cards, " ");
    for (; card != NULL; card = strtok(NULL, " ")) {
      int num = atoi(card);
      if (num == 1) {
        score += 10;
        ones++;
      }
      score += fmin(num, 10);
    }
    while (ones--) {
      if (score > 21) score -= 10;
    }
    if (score > 21) score = 0;
    printf("%d\n", score);
  } 
  return (0);
}