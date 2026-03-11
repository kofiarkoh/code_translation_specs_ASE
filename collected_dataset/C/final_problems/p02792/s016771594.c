#include<stdio.h>
int main(){
  int n,a,b,i,j;
  long long s=0,c[20][20]={};
  scanf("%d",&n);
  for(i=1;i<=n;i++){
    a=b=i%10;
    for(j=i;j;j/=10)b=j%10;
    c[a][b]++;
  }
  for(i=0;i<10;i++){
    for(j=0;j<10;j++)s+=c[i][j]*c[j][i];
  }
  printf("%lld\n",s);
  return 0;
}
