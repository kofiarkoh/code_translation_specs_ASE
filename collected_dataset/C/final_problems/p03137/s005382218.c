#include <stdio.h>
#include <stdlib.h>
int cmp(const void *a,const void *b){
  int A= *(int *)a;
  int B= *(int *)b;
  return A-B;
}
int main(){
  int n,m,ans=0;
  scanf("%d %d",&n,&m);
  if(m==1){
    printf("0\n");
    return 0;
  }
  int a[m],i;
 
   for(i=0;i<m;i++){
  scanf("%d",&a[i]);
  }
  qsort(a,m,sizeof(int),cmp);
  int d,e,f;
  for(i=0;i<m-3;i++){
    d=a[i+1]-a[i];
    e=a[i+2]-a[i+1];
    f=a[i+3]-a[i+2];
    if(d>=e&&e>=f)ans+=e;
    else if(d<=e&&e>=f)ans+=d;
    else if(d<=e&&e<=f)ans+=e;
    else ans+=e;
  }
  if(n<=2) ans+=a[m-1]-a[m-2];
  printf("%d\n",ans);
}