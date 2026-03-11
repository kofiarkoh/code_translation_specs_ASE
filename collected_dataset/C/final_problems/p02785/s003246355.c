#include<stdio.h>
#include<string.h>

int down(const void *a , const void *b){
  return *(int *)b - *(int *)a;
}

int main(void){
  int n , k;
  scanf("%d %d",&n ,&k);

  int h[n];
  for(int i = 0;i < n;i ++){
    scanf("%d",&h[i]);
  }
  qsort(h,n,sizeof(int),down);

  long long ans = 0;

  for(int i = k;i < n;i ++){
    ans += h[i];
  }

  printf("%lld\n",ans);

  return 0;

}