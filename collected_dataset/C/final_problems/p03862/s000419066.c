#include<stdio.h>
#include<string.h>

int main(void) {
  int N, x;
  int a[100010];
  scanf("%d %d", &N, &x);
  int i;
  //a[0] = 0;
  for(i=0;i<N;i++){
    scanf("%d",&a[i]);
  }
  //a[N+1] = 0;
  int ans = 0;
  
  if(a[0]>x){
    ans += a[1] - x;
    a[1] = x;
  }
  /*
  if(a[N]>x){
    ans += a[N] - x;
    a[N] = x;
  */

  int sum, tmp;
  for(i=0;i<N-1;i++){
    sum = a[i] + a[i+1];
    if(sum > x){
      tmp = sum - x;
      ans += tmp;
      a[i+1] -= tmp;
    }
  }

  printf("%d\n",ans);
  
  return 0;
}
