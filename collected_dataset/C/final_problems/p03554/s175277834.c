#include<stdio.h>
#include<stdlib.h>

#define MIN(a,b) ((a)<(b)?(a):(b))

typedef struct range{
  int l,r;
} range;

int cmp(const void *a,const void *b){
  return ((range *)a)->r-((range *)b)->r;
}

void run(void){
  int n;
  scanf("%d",&n);
  int *b=(int *)calloc(n+1,sizeof(int));
  int i;
  for(i=1;i<=n;i++){
    scanf("%d",b+i);
    b[i]+=b[i-1];
  }
  int q;
  scanf("%d",&q);
  range *p=(range *)calloc(q,sizeof(range));
  for(i=0;i<q;i++) scanf("%d%d",&p[i].l,&p[i].r);
  qsort(p,q,sizeof(range),cmp);
  int *dp=(int *)calloc(n+1,sizeof(int));
  int j=0;
  for(i=1;i<=n;i++){
    int local=dp[i-1]+b[i]-b[i-1];
    while(j<q && p[j].r==i){
      int l=p[j].l;
      local=MIN(local,dp[l-1]+(i-l+1)-(b[i]-b[l-1]));
      j++;
    }
    dp[i]=local;
  }
  printf("%d\n",dp[n]);
}

int main(void){
  run();
  return 0;
}
