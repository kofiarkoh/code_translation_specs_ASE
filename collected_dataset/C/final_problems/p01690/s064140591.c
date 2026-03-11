#include<stdio.h>
int MAX(int a,int b){return a<b?b:a;}
int MIN(int a,int b){return a<b?a:b;}
void ifMAX(int *a,int b){*a=MAX(*a,b);}
int main(){
  int h,n,k,m,p,d,i,j,l,a[110],b[110],c[110],e[110],dp[110][110]={};
  int s[100010]={};
  int t[110][11010]={1};
  scanf("%d %d %d %d %d %d",&h,&n,&k,&m,&p,&d);
  for(i=0;i<n;i++)scanf("%d %d",&e[i],&c[i]);
  for(i=0;i<m;i++)scanf("%d %d",&b[i],&a[i]);
  for(i=0;i<10010;i++){
    //if(s[i+1]<s[i])s[i+1]=s[i];
    for(j=0;j<m;j++)ifMAX(&s[i+a[j]],s[i]+b[j]);
  }
  //for(i=0;i<20;i++)printf("%2d %2d\n",i,s[i]);printf("\n");
  for(l=0;l<n;l++){
    for(i=k-1;i>-1;i--){
      for(j=0;j<=h;j++){
	if(t[i][j])ifMAX(&t[i+1][j+e[l]],t[i][j]+c[l]);
      }
    }
  }
  //for(i=0;i<20;i++)printf("%2d %2d %2d\n",i,t[k][i]-1,s[t[k][i]-1]);
  for(i=0;i<=d;i++){
    for(j=h;j;j--){
      ifMAX(&dp[i][j-1],dp[i][j]);
      for(l=0;l<h;l++){
	if(j<l)continue;
	ifMAX(&dp[i+1][MIN(h,j-l+p)],dp[i][j]+s[t[k][l]-1]);
      }
    }
  }
  /*for(i=0;i<=d;i++){
    for(j=0;j<h;j++)printf("%3d ",dp[i][j]);printf("\n");
    }//*/
  printf("%d\n",dp[d][0]?dp[d][0]:-1);
  return 0;
}