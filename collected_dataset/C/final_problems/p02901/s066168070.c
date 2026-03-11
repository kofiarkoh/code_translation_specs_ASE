#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
int main(){
  int n,m,i,a,b,c,j,k,d[100010]={};
  scanf("%d %d",&n,&m);
  n=(1<<n);//printf("%d\n",n);
  for(i=1;i<n;i++)d[i]=1e9+10;
  while(m--){//printf("%d\n",m);
    scanf("%d %d",&a,&b);
    k=0;
    while(b--){//printf("!%d\n",b);
      scanf("%d",&c);
      k+=(1<<(c-1));
    }//printf("!!\n");
    for(j=0;j<n;j++)d[j|k]=MIN(d[j|k],d[j]+a);
  }
  if(d[n-1]>1e9)d[n-1]=-1;
  printf("%d\n",d[n-1]);
  return 0;
}
