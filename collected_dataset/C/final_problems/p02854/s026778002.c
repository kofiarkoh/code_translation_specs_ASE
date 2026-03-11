#include<stdio.h>

int main(void){
    int N;
    int L[200001];
    long int sum = 0, kemo = 0;
    scanf("%d", &N);
    for (int a = 0; a < N;a++){
        scanf("%d", &L[a]);
    }
  if(N%2==0){
      for (int b = 0; b < N / 2;b++){
          sum += L[b];
  }
  for (int c = N / 2; c < N;c++){
      kemo += L[c];
  }
  }else{
      for (int d = 0; d < N / 2;d++){
          sum += L[d];
      }
      for (int e = N / 2 + 1; e < N;e++){
          kemo += L[e];
      }
    
      if(sum>=kemo){
          kemo += L[N / 2];
      }else{
          sum += L[N / 2];
      }
    
  }

  if(sum>=kemo){
      printf("%ld", sum - kemo);
      return 0;
  }else{
      printf("%ld", kemo - sum);
      return 0;
  }
}


