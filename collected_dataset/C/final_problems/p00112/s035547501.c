#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

int cmp(const void *a,const void *b){
  return *(int *)a-*(int *)b;
}

int main(void){
  int n,t[10001],i,ans;
  while(1){
    memset(t,0,sizeof(t));
    scanf("%d",&n);
    if(!n)break;
    t[0]=ans=0;
    for(i=1;i<=n;i++)
      scanf("%d",&t[i]);
    qsort(t,n+1,sizeof(int),cmp);
    for(i=1;i<n;i++){
      t[i]=t[i-1]+t[i];
      ans+=t[i];
    }
    printf("%d\n",ans);
  }
  return 0;
}