#include<stdio.h>
int main(){
  int n,i,j,c[4],d[4],m;
  for(j=0;scanf("%d",&n),n;j++){
    for(i=0;i<4;i++)scanf("%d",&c[i]);
    m=c[0]+c[1]*5+c[2]*10+c[3]*50;
    n=m*10-n;//printf("%d\n",n);
    d[3]=n/500;
    n%=500;
    d[2]=n/100;
    n%=100;
    d[1]=n/50;
    n%=50;
    d[0]=n/10;
    n%=10;
    for(i=0;i<4;i++){//printf("%d ",d[i]);
      if(c[i]&&d[i])c[i]-=d[i];
    }
    if(j)printf("\n");
    if(c[0])printf("10 %d\n", c[0]);
    if(c[1])printf("50 %d\n", c[1]);
    if(c[2])printf("100 %d\n",c[2]);
    if(c[3])printf("500 %d\n",c[3]);
  }
  return 0;
}