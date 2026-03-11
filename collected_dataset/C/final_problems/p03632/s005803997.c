#include<stdio.h>

int main()
{
  int a,b,c,d;
  int max,min;
  int ans;
  scanf("%d%d%d%d",&a,&b,&c,&d);
  if(a>=c){
    max=a;
  }
  else max=c;
  if(b<d){
    min=b;
  }
  else min=d;
  ans=min-max;
  if(ans<=0) ans=0;
  printf("%d",ans);
  return 0;
}
