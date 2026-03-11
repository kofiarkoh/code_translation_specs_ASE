#include <stdio.h>
#define N_MAX 100001

int main(void){
  int n, m, i, j, p, d, pp, dd, ppn, ddn;
  int ret,a[2][N_MAX], parent[N_MAX];
  ret = scanf("%d %d\n",&n,&m);
  for(i=0;i<n+m;i++){
  	ret = scanf("%d %d\n",&a[0][i],&a[1][i]);
  }

  for(i=1;i<=n;i++){
    p=-1;d=-1;
    for(j=0;j<n+m;j++){
      if(a[1][j] == i){
        if(p==-1){
          p=a[0][j];
        }else{
          d=a[0][j];
        }
      }
      fprintf(stderr, "%d-%d:%d %d \n", i, j, p, d);
    }
    fprintf(stderr, "%d:%d %d \n", i, p, d);
    if(p == -1){
      printf("%d\n",0);
    }else if(d == -1){
      printf("%d\n",p);
    }else{
      pp=p;dd=d;
      while(pp != -1 && dd != -1){
        ppn=-1; ddn=-1;
        for(j=0;j<n+m;j++){
          if(a[1][j] == pp){
            if(ppn == -1)
            ppn=a[0][j];
          }
          if(a[1][j] == dd){
            if(ddn == -1)
            ddn=a[0][j];
          }
        }
        pp=ppn, dd=ddn;
      }
      if(pp != -1){
        printf("%d\n",p);
      }else{
        printf("%d\n",d);
      }
    }
  }
  
  return 0;
}
