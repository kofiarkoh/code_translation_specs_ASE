#include <stdio.h>
#include <stdlib.h>

int compare_int(const void *a,const void *b){
  return *(long*)a-*(long*)b;
}

int main(void){
  int N,M;
  scanf("%d%d",&N,&M);

  int A[N+1];
  int B[N+1];
  for(int i = 1;i <= N;i++) scanf("%d",&A[i]);

  A[0] = 0;
  for(int i = 1;i <= N;i++){
    A[i] = (A[i]+A[i-1])%M;
  }

  qsort(A,N+1,sizeof(int),compare_int);

  long long ans = 0;
  int j = 0;
  B[0] = 1;
  for(int i = 1;i <= N;i++){
    if(A[i] == A[i-1]) B[j]++;
    else B[++j] = 1;
  }

  for(int i = 0;i <= j;i++){
    ans += (B[i]*(B[i]-1))/2;
  }

  printf("%lld\n",ans);

  return 0;
}