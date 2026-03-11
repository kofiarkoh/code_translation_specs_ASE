#include<stdio.h>
int MAX(int a,int b){return a<b?b:a;}
int main(){
  int n,m,o,a[350],b[350],i,j,l,k[350]={0};
  int t[350]={0};
  int d[100][350]={0};
  scanf("%d %d %d",&n,&m,&o);
  for(i=0;i<m;i++)scanf("%d %d %d %d",&a[i],&b[i],&k[i],&t[i]);
  for(i=0;i<50;i++){
    for(j=0;j<=o;j++){
      d[i+1][j]=MAX(d[i+1][j],d[i][j]);
      for(l=0;l<m;l++){
	if(i==a[l]*10+b[l])d[i+k[l]][j+1]=MAX(d[i+k[l]][j+1],d[i][j]+t[l]);
      }
    }
  }
  /*for(i=0;i<=50;i++){
    for(j=0;j<=o;j++)printf("%d %d %d\n",i,j,d[i][j]);
    }//*/
  printf("%d\n",d[50][o]);
  return 0;
}