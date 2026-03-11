#include<stdio.h>
int main(){
  long a,v,b,w,t;
  scanf("%ld %ld",&a,&v);
  scanf("%ld %ld",&b,&w);
  scanf("%ld",&t);
  if(b-a+(w*t-v*t)>0)printf("NO\n");
  else printf("YES\n");
  return 0;
}