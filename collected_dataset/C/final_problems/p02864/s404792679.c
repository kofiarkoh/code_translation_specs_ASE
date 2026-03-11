#include<stdio.h>
long long MAX(long long a,long long b){return a<b?b:a;}
long long MIN(long long a,long long b){return a<b?a:b;}
long long d[310][310][310],c[310]={};
int main(){
  int n,m,i,j,k;
  scanf("%d %d",&n,&m);
  for(i=0;i<n;i++)scanf("%lld",&c[i+1]);
  n++;
  for(i=0;i<=n;i++){
    for(j=0;j<=n;j++){
      for(k=0;k<=m;k++)d[i][j][k]=1e18;
    }
  }
  d[0][0][0]=0;
  for(i=1;i<=n;i++){
    for(j=0;j<=i;j++){
      for(k=0;k<=m;k++){
	d[i][i][k  ]=MIN(d[i][i][k  ],d[i-1][j][k]+MAX(c[i]-c[j],0));
	d[i][j][k+1]=MIN(d[i][j][k+1],d[i-1][j][k]);
      }
    }
  }
  /*for(i=0;i<=n;i++){
    for(j=0;j<=n;j++){
      for(k=0;k<=m;k++)printf("%20lld ",d[i][j][k]);
      printf("\n");
    }
    printf("\n");
    }
    printf("\n");//*/
  printf("%lld\n",d[n][n][m]);
  return 0;
}
