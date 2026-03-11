#include<stdio.h>
#define MAX 1001


int max(int a,int b){
if(a>b){return a;}
else{return b;}

}




int func(int V[],int W[],int n,int w){
int dp[n+1][w+1];
int i,j;
int M;

for(i=0;i<n+1;i++){/*初期化*/
 for(j=0;j<w+1;j++){
  dp[i][j]=-1;
 }
}

dp[0][0]=0;

for(i=1;i<=n;i++){

 for(j=0;j<=w;j++){

  if(dp[i-1][j]!=-1){

   if(dp[i-1][j]>dp[i][j])dp[i][j]=dp[i-1][j];/*おろす*/

   if(j+W[i]<=w){
    if(dp[i-1][j]+V[i]>dp[i][j+W[i]]){dp[i][j+W[i]]=dp[i-1][j]+V[i];}
    else{dp[i][j+W[i]]=dp[i-1][j+W[i]];}
   }

  }

 }

}
/*
for(i=0;i<=n;i++){
 for(j=0;j<=w;j++){
  printf("%d ",dp[i][j]);
 }
 printf("\n");
*/




M=-10;
for(i=0;i<=w;i++)M=max(dp[n][i],M);

return M;
}


int main(){
int n,i,x;
int w;
int V[MAX];
int W[MAX];

scanf("%d %d",&n,&w);
for(i=1;i<=n;i++)scanf("%d %d",&V[i],&W[i]);

x=func(V,W,n,w);

printf("%d\n",x);

return 0;
}
