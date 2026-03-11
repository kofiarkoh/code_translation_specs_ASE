#include<stdio.h>
int MAX(int a,int b){return a<b?b:a;}
int main(){
  int n[2],i,j,a,m;
  while(scanf("%d %d",&n[0],&n[1]),n[0]){
    int d[3][30]={};
    for(i=0;i<2;i++){
      for(j=0;j<n[i];j++){
	scanf("%d",&a);
	d[i][a]++;
      }
    }
    for(i=m=0;i<25;i++)m+=MAX(d[0][i],d[1][i])*i;
    printf("%d\n",m);
  }
  return 0;
}