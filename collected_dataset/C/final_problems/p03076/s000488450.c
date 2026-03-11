#include<stdio.h>

#define MAX 5

int main() {
  int dish[MAX];
  int digit_1 = 9;
  int lastorder;
  int time = 0;
  int i;

  for(i = 0; i < MAX; i++) scanf("%d", &dish[i]);

  for(i = 0; i < MAX; i++) {
    if(dish[i] % 10 != 0 && dish[i] % 10 < digit_1 + 1) {
      digit_1 = dish[i] % 10;
      lastorder = i;
    }
  }

  for(i = 0; i < MAX; i++) {
    if(i != lastorder) {
      time += dish[i];
      if(time % 10 != 0) {
        time /= 10;
        time = time * 10 + 10;
      }
    }
  }
  time += dish[lastorder];

  printf("%d\n", time);

  return 0;
}
