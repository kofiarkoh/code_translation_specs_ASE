#include <stdio.h>

int main(void){
  int N,ans = 0;
  scanf("%d\n",&N);
  char A[N],B[N],C[N];

  scanf("%s%s%s",A,B,C);

  for(int i = 0;i < N;i++){
    if(A[i] == B[i] && B[i] == C[i]){
    }else{
      if(A[i] != B[i] && B[i] != C[i] && C[i] != A[i]){
        ans += 2;
      }else{
        ans++;
      }
    }
  }

  printf("%d\n",ans);
  return 0;
}