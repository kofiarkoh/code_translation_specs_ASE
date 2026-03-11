#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
int zMIN(int a,int b){return a*b?MIN(a,b):a+b;}
int main(){
  int n,m,ta,tb,s,i,j,k,min,a,b,c;
  while(scanf("%d %d %d %d %d",&n,&m,&s,&ta,&tb),n){
    int d[110][110]={0};
    for(i=0;i<m;i++){
      scanf("%d %d %d",&a,&b,&c);
      d[a][b]=d[b][a]=c;
    }//printf("a\n");
    for(k=0;k<n;k++){
      for(i=0;i<n;i++){
	for(j=0;j<n;j++)d[i][j]=zMIN(d[i][j],d[i][k]+d[k][j]);
      }
    }
    for(i=min=0;i<n;i++)min=zMIN(min,d[i][s]+d[i][ta]+d[i][tb]);
    printf("%d\n",min);
  }
  return 0;
}