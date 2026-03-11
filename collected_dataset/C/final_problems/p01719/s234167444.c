#include<stdio.h>
int main(){
  int n,m,o,j,k,max,mi,a,b[20],d[15][100010]={0};
  scanf("%d %d %d",&n,&m,&o);
  for(j=0;j<n;j++)scanf("%d",&b[j+1]);
  while(--m){//printf("m%d\n",m);
    for(j=1;j<=n;j++){
      scanf("%d",&a);//printf("%d\n",b[j]);
      for(k=0;k<=o;k++){
	d[j][k]=d[j-1][k];
	if(k<b[j])continue;
	if(d[j][k]<d[j][k-b[j]]+a)d[j][k]=d[j][k-b[j]]+a;
      }
      b[j]=a;
      //for(k=0;k<=o;k++)printf("%d ",d[j][k]);printf("\n");
    }
    for(k=max=0;k<=o;k++){
      if(max<d[n][k]-k)max=d[n][mi=k]-k;
    }
    o+=max;
  }//printf("\n");
  printf("%d\n",o);
  return 0;
}