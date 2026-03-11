#include<stdio.h>
#include<stdlib.h>

typedef long long int int64;

int cmp(const void *a,const void *b){
  return *(int *)a-*(int *)b;
}

void run(void){
  int w,h;
  scanf("%d%d",&w,&h);
  int *p=(int *)calloc(w,sizeof(int));
  int *q=(int *)calloc(h,sizeof(int));
  int i,j;
  for(i=0;i<w;i++) scanf("%d",p+i);
  for(i=0;i<h;i++) scanf("%d",q+i);
  qsort(p,w,sizeof(int),cmp);
  qsort(q,h,sizeof(int),cmp);
  int64 ans=0;
  i=j=0;
  while(i<w && j<h){
    if(p[i]<=q[j]){
      ans+=(int64)p[i++]*(h+1-j);
    } else {
      ans+=(int64)q[j++]*(w+1-i);
    }
  }
  while(i<w) ans+=p[i++];
  while(j<h) ans+=q[j++];
  printf("%lld\n",ans);
}

int main(void){
  run();
  return 0;
}
