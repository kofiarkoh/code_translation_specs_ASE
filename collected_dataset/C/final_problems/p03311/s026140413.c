#include <stdio.h>
#include <stdlib.h>

int compare_int(const void *a,const void *b){
  if(*(int*)a-*(int*)b > 0) return 1;
  return -1;
}

int main(void){
  int N;
  scanf("%d",&N);

  int A[N],B[N];
  for (int i = 0; i < N; i++) {
    scanf("%d",&A[i]);
    B[i] = A[i]-i-1;
  }

  qsort(B,N,sizeof(int),compare_int);

  int b = B[N/2-1];
  int ans = 0;
  for (int i = 0; i < N; i++) {
    ans += abs(A[i]-b-i-1);
  }

  printf("%d\n",ans);

  return 0;
}