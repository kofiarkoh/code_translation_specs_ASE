#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
int MAX(int a,int b){return a<b?b:a;}
int main(){
  int n,g,m,i,j,k,s[20],e[3][60],nx;
  while(scanf("%d %d %d",&n,&g,&m),n){
    double a=0,d[60][6500]={1};
    for(i=0;i<n;i++)scanf("%d",&s[i]);
    while(m--){
      scanf("%d %d %d",&i,&j,&k);
      e[j-1][i]=k;
    }
    for(i=0;i<g;i++){
      for(j=0;j<6000;j++){//printf("%lf ",d[i][j]);
	for(k=0;k<n;k++){
	  nx=MIN(g,i+s[k]);
	  nx=MIN(g,nx+e[0][nx]);
	  d[nx][MAX(0,j+e[1][nx]-e[2][nx])]+=d[i][j]/n;
	}
      }//printf("\n");
    }
    for(i=0;i<6000;i++)a+=i*d[g][i];
    printf("%.0f\n",a);
  }
  return 0;
}