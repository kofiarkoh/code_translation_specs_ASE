#include<stdio.h>
#include<stdlib.h>

int cmp(const void *a,const void *b){
  return *(int *)a-*(int *)b;
}

void run(void){
  int n,k,q;
  scanf("%d%d%d",&n,&k,&q);
  int *a=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",a+i);
  int *tmp=(int *)calloc(n,sizeof(int));
  int *pop=(int *)calloc(n,sizeof(int));
  int ans=1000000000;
  for(i=0;i<n-q;i++){
    int plen=0;
    int j=0;
    while(j<n){
      int p=j;
      int tlen=0;
      while(p<n && a[p]) tmp[tlen++]=a[p++];
      qsort(tmp,tlen,sizeof(int),cmp);
      for(p=0;p<tlen-k+1;p++) pop[plen++]=tmp[p];
      j+=tlen+1;
    }
    if(plen<q) break;
    qsort(pop,plen,sizeof(int),cmp);
    int t=pop[q-1]-pop[0];
    if(ans>t) ans=t;
    for(j=0;!a[j];j++);
    int min=j;
    for(j++;j<n;j++) if(!a[j] && a[j]<a[min]) min=j;
    a[j]=0;
  }
  printf("%d\n",ans);
}

int main(void){
  run();
  return 0;
}
