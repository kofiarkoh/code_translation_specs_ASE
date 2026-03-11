#include<stdio.h>

int main (void) {
  int A[100010];
  int N;
  int i,j;
  int x;
  scanf("%d",&N);
  for(i=1;i<=N;i++){
    scanf("%d",&x);
    A[i] = x;
  }

  int pos = 0;
  int sum = 0;
  int num[100010] = {0};
  int flag = 0;
  for(i=1;i<=N;i++){

    pos++;
    if(flag){
      num[pos] = num[pos-1];
      flag = 0;
    } else {
      num[pos] = num[pos-1] + 1;
    }

    sum ++;
    if(A[i]-sum<sum-1){
      flag = 1;
      sum--;
    } 
    
  }
      
  long long int ans = 1;
  for(i=1;i<=N;i++){
    ans *= num[i];
    ans %= 1000000007;
  }

  printf("%lld\n",ans);
  
  return 0;
}