#include<stdio.h>

void insubunkai(int N,int *A){
  int i;
  for(i=2;i<=N;i++){
    if(N%i == 0){
      A[i]++;
      N /= i;
      i = 1;
    }
  }
}

int main(){
  int N;
  int A[1001]={};
  long long int count=1;
  int i;

  scanf("%d", &N);
  for(i=1;i<=N;i++){
    insubunkai(i,A);
  }
  for(i=0;i<=N;i++){
    if(A[i]>=1){
      count *= A[i]+1;
      count %= 1000000007;
    }
  }
  printf("%lld\n", count);
  return 0;
}
