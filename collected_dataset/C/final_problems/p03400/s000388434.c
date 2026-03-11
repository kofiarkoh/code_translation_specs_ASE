#include<stdio.h>
int main(){
  int n,d,x;
  scanf("%d%d%d",&n,&d,&x);
  int a[n+1],i;
  for(i=0;i<n;i++){
    scanf("%d",&a[i]);
    x+=(d-1)/a[i]+1;
  }
  printf("%d\n",x);
  return 0;
}
