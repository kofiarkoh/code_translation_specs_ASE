#include<stdio.h>

int main(){
  int candy;

  while(scanf("%d", &candy) != EOF){

    while(candy > 39)
      candy -= 39;

    if(candy < 10)
      printf("3C0%d\n", candy);
    else
      printf("3C%d\n", candy);
  }

  return 0;
}