#include<stdio.h>
#define N 100
#define INF 2140000000
#define min(x,y) ((x)<=(y)?(x):(y))

int n;

int matrixchainmultiplication(int *a){
  int i,j,k,l,m[N+1][N+1];

  for(i=1;i<=n;i++) m[i][i]=0;
  for(i=2;i<=n;i++){
    for(j=1;j<=n-i+1;j++){
      k=j+i-1;
      m[j][k]=INF;
      for(l=j;l<=k-1;l++){
	m[j][k]=min(m[j][k],m[j][l]+m[l+1][k]+a[j-1]*a[l]*a[k]);
      }
    }
  }
      return m[1][n];
}

int main(){
  int a[N+1],i;

  scanf("%d",&n);
  for(i=0;i<n;i++) scanf("%d%d",&a[i],&a[i+1]);
  
  printf("%d\n",matrixchainmultiplication(a));
  
    return 0;
}


