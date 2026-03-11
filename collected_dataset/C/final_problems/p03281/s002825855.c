#include <stdio.h>
#include <stdlib.h>
#include <math.h>

//numberが素数ならば1，そうでなければ0を返す関数
int primality(int number);

int main(void){
  int N;
  int i, j, k;
  int count = 0;

  scanf("%d", &N);

  for(i = 3; i <= N; i++){
    if(primality(i) == 1){
      if(((double) i) <= pow((double) N, 1.0/7.0)){
        count++;
      }
    }
  }

  for(i = 3; i <= N; i++){
    for(j = i+1; j <= N; j++){
      if(primality(i) == 1 && primality(j) == 1){
        if((i*j*j*j) <= N){
          count++;
        }
        if((i*i*i*j) <= N){
          count++;
        }
      }
    }
  }

  for(i = 3; i <= N; i++){
    for(j = i+1; j <= N; j++){
      for(k = j+1; k <= N; k++){
        if(primality(i) == 1 && primality(j) == 1 && primality(k) == 1){
          if((i*j*k) <= N){
            count++;
          }
        }
      }
    }
  }

  printf("%d\n", count);
  return 0;
}

int primality(int number){
  int i;

  if(number <= 1){
    return 0;
  }
  else{
    for(i = 2; i < number; i++){
      if(number%i == 0){
        return 0;
      }
    }
    return 1;
  }
}
