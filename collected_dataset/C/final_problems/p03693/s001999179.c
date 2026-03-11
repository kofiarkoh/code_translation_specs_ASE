#include<stdio.h>

int main()
{
  int r,g,b,tmp,R,G,ans;

  scanf("%d %d %d",&r,&g,&b);

  R=100*r;
  G=10*g;
  tmp=R+G+b;

  ans=tmp%4;

  if(ans==0)printf("YES");
  else printf("NO");

  return 0;
}
