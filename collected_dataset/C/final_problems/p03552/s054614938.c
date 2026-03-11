#include<stdio.h>
#include<stdlib.h>
int main(){
  int n,a,b,d[100010],i,j,mi;
  scanf("%d %d %d",&n,&a,&b);
  for(i=0;i<n;i++)scanf("%d",&d[i]);
  for(i=0;i<n;i++){
    mi=i;
    for(j=i;j<n;j++){
      if(d[mi]<d[j])mi=j;
    }
    a=d[mi];
    for(j=mi=i+1;j<n;j++){
      if(d[mi]>d[j])mi=j;
    }
    if(j<n)b=d[mi];
    i=mi;
  }
  printf("%d\n",abs(a-b));
  return 0;
}
