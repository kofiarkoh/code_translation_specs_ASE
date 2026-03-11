#include<stdio.h>
int MIN(int a,int b){
  if(a==0||a>b)return b;
  return a;
}
int main(){
  int Y[]={-1,0,1,0};
  int X[]={0,1,0,-1};
  int w,h,i,j,k,l,ny,nx,ax[15],ay[15],x[900],y[900],r,t,rr,min;
  while(scanf("%d %d",&w,&h),w||h){
    char d[30][30]={0};
    int dp[10][1<<10]={0};
    int m[30][30]={0};
    int f[30][30]={0};
    int c[30][30]={0};
    r=0;
    for(i=rr=1;i<=h;i++){
      for(j=1;j<=w;j++){
	scanf(" %c",&d[i][j]);
	if(d[i][j]=='o')d[ay[0]=i][ax[0]=j]='.';
	if(d[i][j]=='*'){
	  d[ay[rr]=i][ax[rr]=j]='.';
	  rr++;
	}
      }
    }
    for(i=min=0;i<rr;i++){
      f[y[t=0]=ay[i]][x[t]=ax[i]]=i+1;
      c[y[t]][x[t]]=0;
      for(r=1;r-t;t++){
	for(j=0;j<4;j++){
	  ny=y[t]+Y[j];
	  nx=x[t]+X[j];
	  if(d[ny][nx]-'.'||f[ny][nx]==i+1)continue;
	  c[y[r]=ny][x[r]=nx]=c[y[t]][x[t]]+1;
	  f[ny][nx]=i+1;
	  r++;
	}
      }
      for(j=1;j<rr;j++)m[i][j]=c[ay[j]][ax[j]];
    }
    rr--;
    for(i=0;i<rr;i++){
      dp[i][1<<i]=m[0][i+1];
      if(m[0][i+1]==0)rr=0;
    }
    for(l=0;l<rr;l++){
      for(i=0;i<rr;i++){
	for(j=0;j<1<<rr;j++){
	  if(dp[i][j]==0)continue;
	  for(k=0;k<rr;k++){
	    if(j&1<<k)continue;
	    dp[k][j|1<<k]=MIN(dp[k][j|1<<k],dp[i][j]+m[i+1][k+1]);
	  }
	}
      }
    }
    for(i=min=0;i<rr;i++)min=MIN(min,dp[i][(1<<rr)-1]);
    if(min)printf("%d\n",min);
    else   printf("-1\n");
  }
  return 0;
}
    