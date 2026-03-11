#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
int main(){
  int n,m,p[1010],d[1010][1010],w[2][310][310];
  int a,b,c,i,j,k,l,t,M=100000000;
  char r;
  while(scanf("%d %d",&n,&m),n+m){
    for(i=0;i<n*n;i++)w[0][i/n][i%n]=w[1][i/n][i%n]=M;
    for(i=0;i<1010;i++){
      for(j=0;j<1010;j++)d[i][j]=M;
    }
    for(i=0;i<n;i++)w[0][i][i]=w[1][i][i]=0;
    while(m--){
      scanf("%d %d %d %c",&a,&b,&c,&r);
      t=(r=='L')?0:1;
      w[t][a-1][b-1]=w[t][b-1][a-1]=c;
    }
    for(i=0;i<2;i++){
      for(j=0;j<n;j++){
	for(k=0;k<n;k++){
	  for(l=0;l<n;l++)w[i][k][l]=MIN(w[i][k][l],w[i][k][j]+w[i][j][l]);
	}
      }
    }
    /*for(i=0;i<2;i++){
      for(j=0;j<n;j++){
	for(k=0;k<n;k++)printf("%10d ",w[i][j][k]);printf("\n");
      }printf("\n");
      }//*/
    scanf("%d",&l);
    for(i=0;i<l;i++)scanf("%d",&p[i]);
    for(i=0;i<l;i++)p[i]--;
    d[0][p[0]]=0;
    for(i=0;i<l-1;i++){
      for(j=0;j<n;j++){
	if(d[i][j]==M)continue;
	d[i+1][j]=MIN(d[i+1][j],d[i][j]+w[0][p[i]][p[i+1]]);
	for(k=0;k<n;k++){
	  d[i+1][k]=MIN(d[i+1][k],d[i][j]+w[0][p[i]][j]+w[1][j][k]+w[0][k][p[i+1]]);
	}
      }
    }
    for(i=0;i<n;i++)d[l-1][0]=MIN(d[l-1][0],d[l-1][i]);
    printf("%d\n",d[l-1][0]);
  }
  return 0;
}