#include <stdio.h>

int main(){
  int a;
  char s[50];

  scanf("%d", &a);

  scanf("%s", s);

  if (a < 3200){

    printf("red\n");
  } else {

    printf("%s\n", s);
  }

  return 0;
}