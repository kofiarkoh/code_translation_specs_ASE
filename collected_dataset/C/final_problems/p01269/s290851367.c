#include<stdio.h>
#define M 1000000000
int MIN(int a,int b){return a<b?a:b;}
int main(){
  int n,m,l,min,mi,i,j,k,f[110]={0};
  int a,b,d[110][110],t[110][110],c[110][110]={0};
  while(scanf("%d %d %d",&n,&m,&l),n){
    for(i=0;i<110;i++){
      for(j=0;j<110;j++)d[i][j]=c[i][j]=M;
      f[i]=-1;
    }
    d[0][1]=0;
    while(m--){
      scanf("%d %d %d %d",&a,&b,&i,&j);
      c[b][a]=c[a][b]=i;
      t[b][a]=t[a][b]=j;
    }
    for(i=0;i<=l;i++){
      for(j=1;j<=n;j++){
	min=M;
	for(k=1;k<=n;k++){
	  if(f[k]==i)continue;
	  if(min>d[i][k])min=d[i][mi=k];
	}
	f[mi]=i;
	for(k=1;k<=n;k++){
	  if(c[mi][k]==M)continue;
	  d[i  ][k]=MIN(d[i][k],d[i  ][mi]+t[mi][k]);
	  d[i+1][k]=MIN(d[i][k],d[i+1][k]);
	  if(i+c[mi][k]<=l)d[i+c[mi][k]][k]=MIN(d[i+c[mi][k]][k],d[i][mi]);
	}
      }
    }
    /*for(i=0;i<=l;i++){printf("%2d:",i);
      for(j=0;j<n;j++)printf("%d ",d[i][j+1]);printf("\n");
      }//*/
    printf("%d\n",d[l][n]);
  }
  return 0;
}