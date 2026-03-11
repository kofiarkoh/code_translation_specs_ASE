#include<stdio.h>
int MAX(int a,int b){return a<b?b:a;}
int MIN(int a,int b){return a<b?a:b;}
int n,w,a,i,j,k,max=0,m[110][2],d[110][110][410]={1};
int main(){
  scanf("%d %d",&n,&w);
  for(i=0;i<n;i++)scanf("%d %d",&m[i][0],&m[i][1]);
  a=m[0][0];
  for(i=0;i<n;i++)m[i][0]-=a;
  for(i=0;i<n;i++){
    for(j=i;j>-1;j--){
      for(k=0;k<=400;k++){
	if(d[i][j][k]==0)continue;
	d[i+1][j][k]=d[i][j][k];
	d[i+1][j+1][k+m[i][0]]=MAX(d[i+1][j+1][k+m[i][0]],d[i][j][k]+m[i][1]);
      }
    }
  }
  for(i=0;i<=n;i++){
    for(j=0;j<400;j++){
      if(i*a+j<=w)max=MAX(max,d[n][i][j]);
    }
  }
  printf("%d\n",max-1);
  return 0;
}
