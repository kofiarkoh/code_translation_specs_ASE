#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
int main(){
  int n,m,ta,tb,s,i,j,k,min,a,b,c;
  while(scanf("%d %d %d %d %d",&n,&m,&s,&ta,&tb),n){
    int d[110][110]={0};
    for(i=0;i<110*110;i++)min=d[i/110][i%110]=100000000;
    for(i=0;i<m;i++){
      scanf("%d %d %d",&a,&b,&c);
      d[a-1][b-1]=d[b-1][a-1]=c;
    }//printf("a\n");
    for(i=0;i<n;i++)d[i][i]=0;
    for(k=0;k<n;k++){
      for(i=0;i<n;i++){
	for(j=0;j<n;j++)d[i][j]=MIN(d[i][j],d[i][k]+d[k][j]);
      }
    }
    /*for(i=0;i<n;i++){
      for(j=0;j<n;j++)printf("%2d ",d[i][j]);printf("\n");
      }*/
    for(i=0;i<n;i++)min=MIN(min,d[i][s-1]+d[i][ta-1]+d[i][tb-1]);
    printf("%d\n",min);
  }
  return 0;
}