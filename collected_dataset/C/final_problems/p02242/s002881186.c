#include <stdio.h>

#define N 1000
#define MAX 1000000

 int main(){
   int n, i, j, u, k, v, min;
    int d[N], c[N], g[N][N];
      
    scanf("%d", &n);
      
    for( i = 0; i < n; i++){
        for( j = 0; j < n; j++){
            g[i][j]=MAX;
        }
    }
      
    for( i = 0; i < n; i++){
        d[i] = MAX;
        c[i] = 0;
        scanf("%d%d", &u, &k);
        for( j = 0; j < k; j++){
            scanf("%d", &v);
            scanf("%d", &g[u][v]);
        }
    }
    
	d[0] = 0;
      
    while(1){
      
	  min=MAX;
        
	for(i=0;i<n;i++){
        
	    if(d[i]<min && c[i]==0){
                min=d[i];
                u=i;
            }
        }
          
        if( min == MAX)break;
        c[u] = 1;
          
     for( v = 0; v < n; v++){
            if( d[v] > d[u] + g[u][v]){
                d[v] =d [u] + g[u][v];
            }
        }
    }
    for(i = 0;i < n; i++){
        printf("%d %d\n", i, d[i]);
    }
    return 0;
}