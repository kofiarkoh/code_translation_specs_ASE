#include<stdio.h>

int main(void){
  int i;
  int n,cur;
  int r,t;

  for(;;){
    scanf("%d",&n);
    if(n==0)
      break;

    cur=0;

    for(r=0;r*r*r<=n;r++)
      for(t=0;t*(t+1)*(t+2)/6<=n;t++)
        if(cur<r*r*r+t*(t+1)*(t+2)/6 &&
            r*r*r+t*(t+1)*(t+2)/6<=n)
          cur=r*r*r+t*(t+1)*(t+2)/6;

    printf("%d\n",cur);
  }
  return 0;
}