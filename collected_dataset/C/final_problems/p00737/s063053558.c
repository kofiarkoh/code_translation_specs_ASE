#include<stdio.h>
#define M 1000000000
int main(){
  int Y[]={-1,0,1,0};
  int X[]={0,1,0,-1};
  int dp[4][40][40],w,h,i,j,k,l,mi,mj,mk,min,d[40][40],m,ny,nx,c[4];
  while(scanf("%d %d",&w,&h),w||h){
    int f[4][40][40]={0};
    for(i=0;i<4;i++){
      for(j=0;j<40;j++){
	for(k=0;k<40;k++)dp[i][j][k]=M;
      }
    }
    for(i=1;i<=h;i++){
      for(j=1;j<=w;j++)scanf("%d",&d[i][j]);
    }
    for(i=0;i<4;i++)scanf("%d",&c[i]);
    dp[1][1][1]=0;
    for(l=0;l<4*w*h;l++){
      min=2000000000;
      for(i=0;i<4;i++){
	for(j=1;j<=h;j++){
	  for(k=1;k<=w;k++){
	    if(f[i][j][k])continue;
	    if(min>dp[i][j][k])min=dp[mi=i][mj=j][mk=k];
	  }
	}
      }
      f[mi][mj][mk]=1;
      if(d[mj][mk]-4){
	m=(mi+d[mj][mk]+4)%4;
	ny=mj+Y[m];
	nx=mk+X[m];
	if(dp[m][ny][nx]>dp[mi][mj][mk])dp[m][ny][nx]=dp[mi][mj][mk];
      }
      for(i=0;i<4;i++){
	ny=mj+Y[(mi+i)%4];
	nx=mk+X[(mi+i)%4];
	if(dp[(mi+i)%4][ny][nx]>dp[mi][mj][mk]+c[i])
	   dp[(mi+i)%4][ny][nx]=dp[mi][mj][mk]+c[i];
      }
    }
    min=M;
    for(i=0;i<4;i++){
      if(min>dp[i][h][w])min=dp[i][h][w];
    }
    printf("%d\n",min);
  }
  return 0;
}