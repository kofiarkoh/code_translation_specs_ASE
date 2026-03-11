#include<stdio.h>

int main(){
  int ans=0;
  long X,Y;
  scanf("%ld%ld",&X,&Y);

  while (X<=Y)
  {
    X*=2;
    ans++;
  }
  
  printf("%d",ans);

  return 0;
}