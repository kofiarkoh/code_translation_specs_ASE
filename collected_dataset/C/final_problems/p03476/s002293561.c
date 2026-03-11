#include<stdio.h>
#define N 100001
int main(){
  int i,j,q,l,r,p[N]={},n[N];
  for(i=2;i<N;i++)p[i]=1;
  for(i=2;i<N;i++)if(p[i])for(j=2;i*j<N;j++)p[i*j]=0;
  n[0]=0;for(i=1;i<N;i++)n[i]=n[i-1]+p[i]*p[(i+1)/2];
  scanf("%d",&q);
  for(i=0;i<q;i++){scanf("%d%d",&l,&r);printf("%d\n",n[r]-n[l-1]);}
}
