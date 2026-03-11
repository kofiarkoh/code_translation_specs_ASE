#include<stdio.h>
int MAX(int a,int b){return a<b?b:a;}
int MIN(int a,int b){return a<b?a:b;}
void ifMAX(int *a,int b){*a=MAX(*a,b);}
int main(){
  int h,n,k,m,p,d,i,j,l,a[110],b[110],c[110],e[110],dp[110][110]={};
  int s[11010]={};
  int t[110][11010]={1};
  scanf("%d %d %d %d %d %d",&h,&n,&k,&m,&p,&d);
  for(i=0;i<n;i++)scanf("%d %d",&e[i],&c[i]);
  for(i=0;i<m;i++)scanf("%d %d",&b[i],&a[i]);
  for(i=0;i<10010;i++){
    for(j=0;j<m;j++)ifMAX(&s[i+a[j]],s[i]+b[j]);
  }
  //for(i=0;i<20;i++)printf("%2d %2d\n",i,s[i]);printf("\n");
  for(l=0;l<n;l++){
    for(i=k-1;i>-1;i--){
      for(j=h-1;j>-1;j--){
	if(t[i][j])ifMAX(&t[i+1][j+e[l]],t[i][j]+c[l]);
      }
    }
  }
  //for(i=0;i<40;i++)printf("%2d %2d %2d\n",i,t[k][i]-1,s[t[k][i]-1]);
  for(i=0;i<=d;i++){
    for(j=h;j;j--){
      ifMAX(&dp[i][j-1],dp[i][j]);
      if(dp[i][j]==0&&i)continue;
      for(l=0;l<=j;l++){
	if(t[k][l])ifMAX(&dp[i+1][MIN(h,j-l+p)],dp[i][j]+s[t[k][l]-1]);
      }
    }
  }
  for(i=j=0;i<=h;i++)ifMAX(&j,dp[d][i]);
  /*for(i=0;i<=d;i++){
    for(j=0;j<h;j++)printf("%3d ",dp[i][j]);printf("\n");
    }//*/
  printf("%d\n",j?j:-1);
  return 0;
}