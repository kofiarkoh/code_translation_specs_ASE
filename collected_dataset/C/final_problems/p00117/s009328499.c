#include<stdio.h>
#define rep(i,n) for(i=0;i<n;i++)
int min(int a,int b){return a<b?a:b;}
int main(void)
{
  int n,m,mat[20][20],i,j,k,a,b,c,d;
  scanf("%d%d",&n,&m);
  rep(i,n)rep(j,n)mat[i][j]=i-j?1<<21:0;
  rep(i,m){
    scanf("%d,%d,%d,%d",&a,&b,&c,&d);a--,b--;
    mat[a][b]=c;mat[b][a]=d;
  }
  rep(i,n)rep(j,n)rep(k,n)mat[j][k]=min(mat[j][k],mat[j][i]+mat[i][k]);
  scanf("%d,%d,%d,%d",&a,&b,&c,&d);a--,b--;
  printf("%d\n",c-mat[a][b]-mat[b][a]-d);
  return 0;
}