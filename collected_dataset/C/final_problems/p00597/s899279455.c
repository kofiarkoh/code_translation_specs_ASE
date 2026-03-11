#include<stdio.h>
int main(){
  int n,a=4,i,b=3;
  int d[31]={0,1,2};
  for(i=3;i<30;i+=2){
    d[i]=d[i-2]+a;
    d[i+1]=d[i]+b;
    a*=3;
    b+=6;
  }
  while(scanf("%d",&n)!=EOF)printf("%d\n",d[n]);
  return 0;
}