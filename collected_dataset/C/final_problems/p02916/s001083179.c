#include <stdio.h>

int main(){
  int n,a[21],b,c[21],i,sum=0;
  
  scanf("%d",&n);
  for(i=1;i<=n;i++)
    scanf("%d",&a[i]);
  for(i=1;i<=n;i++){
    scanf("%d",&b);
    sum += b;
  }
  for(i=1;i<n;i++)
    scanf("%d",&c[i]);
  
  for(i=1;i<n;i++)
    if(a[i]+1==a[i+1])
      sum += c[a[i]];
  
  printf("%d\n",sum);
  return 0;
}