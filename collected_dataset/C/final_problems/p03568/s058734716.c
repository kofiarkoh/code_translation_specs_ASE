#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>

int main(){
  int N;
  scanf("%d",&N);
  int n;
  int A[N];
  int odd_count=0;
  int even_count=0;
  for(n=0;n<N;n++){
    scanf("%d",&A[n]);
  }
  for(n=0;n<N;n++){
    if(A[n] % 2 ==0){
      even_count++;
    }
    else{
      odd_count++;
    }
  }
  printf("%d",(int)(pow(3,N)-pow(2,even_count)));


}
