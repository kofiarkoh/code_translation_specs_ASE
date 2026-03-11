#include <stdio.h>
int parm(int N, int r)
{
    int pb = 1;
    int p;
    for(int i=1; i<=r; i++){
        p = pb * (N-i+1);
        pb = p;
    }
    return p;
}
int comb(int N, int K)
{
    int t = 1;
    for(int i=2;i<=K;i++)t*=i;
    return parm(N,K) / t;
}
int main(){
  int N;
  int A[100001];
  int D[100001] = {0};
  int max = 0;
  scanf("%d", &N);
  for(int i=0; i<N; i++){
    scanf("%d", &A[i]);
    D[A[i]]++;
    if(max < D[A[i]])max = D[A[i]];
  }
  int ans = 0;
  for(int i=1; i<=max; i++){
    if(D[i]>0){
      ans += comb(D[i],2);
    }
  }
  for(int i=0; i<N; i++){
    printf("%d\n",ans - D[A[i]] + 1);
  }
}