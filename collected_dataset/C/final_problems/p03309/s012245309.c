#include<stdio.h>
#include<stdlib.h>
int desc(const void *a, const void *b) {
  return *(int *)b - *(int *)a;
}
int main(){
  int n,a[200000],ans,b,i;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d",&a[i]);
    a[i]-=i;
  }
  qsort(a,n,sizeof(int),desc);
  b=a[(n-1)/2+1];
  ans=0;
  for(i=0;i<n;i++)
    ans+=abs(a[i]-b);
  printf("%d\n",ans);
  return 0;
}