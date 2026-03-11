#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

int main(void){
  int N, i, n, f=0, c=0;
  scanf("%d",&N);
  for(i=1;i<=N;i++){
    scanf("%d",&n);
    if(f==1) f=0;
    else if(n==i){c++; f=1;} 
  }
  printf("%d\n",c);
  return 0;
}
