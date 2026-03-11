#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
int main(){
  int i,j,n,min;
  while(scanf("%d",&n),n){
    min=1000000000;
    for(i=0;i*i*i<=n;i++);
    while(i--){
      for(j=0;j*j<=n-i*i*i;j++);
      while(j--)min=MIN(min,i+j+n-i*i*i-j*j);
    }
    printf("%d\n",min);
  }
  return 0;
}