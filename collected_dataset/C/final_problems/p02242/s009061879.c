#include <stdio.h>

#define inf 1e9
#define max 150
#define min(a,b) ((a) < (b) ? (a) : (b))

int main(){
  int n,i,u,k,j,v,c;
  int d[max][max];

  for(i=0;i<max;i++){
    for(j=0;j<max;j++){
      d[i][j] = inf;
    }
    d[i][i] = 0;
  }
  
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d%d",&u,&k);
    for(j=0;j<k;j++){
      scanf("%d%d",&v,&c);
      d[u][v] = c;
    }
  } 

  for(k=0;k<n;k++){
    for(i=0;i<n;i++){
      for(j=0;j<n;j++){
	d[i][j] = min(d[i][j],d[i][k]+d[k][j]);
      }
    }
  }

  for(i=0;i<n;i++){
    printf("%d %d\n",i,d[0][i]);
  }

  return 0;
}

