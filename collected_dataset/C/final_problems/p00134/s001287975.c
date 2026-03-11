#include<stdio.h>

int main(){

  int n, sum, in, i;

  sum = 0;

  scanf("%d",&n);

  for(i = 0; i < n; i++){
    scanf("%d",&in);
    sum += in;
  }

  printf("%d\n",sum/n);

  return 0;

}