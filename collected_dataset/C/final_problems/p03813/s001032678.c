#include <stdio.h>

int main() {
  int rating;
  scanf("%d", &rating);
  if(rating < 1200) {
    printf("ABC\n");
  } else {
    printf("ARC\n");
  }
  return 0;
}
