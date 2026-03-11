#include <stdio.h>
int main(void){
  int i;
  int n,m;
  int x;
  int a[100001];
  int b[100001];
  scanf("%d %d",&n,&m);
  for(i=0;i<m;i++){
    scanf("%d %d",&a[i],&b[i]);
  }
  if(n==m){
    x=0;
  }
  else{
    x=n-m-1;
  }
  printf("%d",x);
  return 0;
}