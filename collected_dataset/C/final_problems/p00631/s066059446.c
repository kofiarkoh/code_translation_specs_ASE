#include <stdio.h>

long a1045(long *a,long p,long g,int n){
  long x,y;
  if(n==0) return (p>g)?(p-g):(g-p);

  if((x=a1045(a+1,p+a[0],g,n-1))==0) return 0;
  if((y=a1045(a+1,p,g+a[0],n-1))==0) return 0;
  return (x<y)?x:y;
}
int main(){
  long a[20];
  int i,n;
  while(scanf("%d",&n)){
    if(n==0) break;
    for(i=0;i<n;i++) scanf("%ld",&a[i]);
    printf("%ld\n",a1045(&a[0],0,0,n));
  }
  return 0;
}