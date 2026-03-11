#include<stdio.h>
int main(){
  int n;
  scanf("%d",&n);
  int l[n+1],i,j,k,count=0;
  for(i=0;i<n;i++)
    scanf("%d",&l[i]);
  for(i=0;i<n;i++)
    for(j=i+1;j<n;j++)
      for(k=j+1;k<n;k++)
        if(l[i]+l[j]>l[k] && l[i]+l[k]>l[j] && l[i]<l[j]+l[k] && l[i]!=l[j] && l[j]!=l[k] && l[k]!=l[i])
          count++;
  printf("%d\n",count);
  return 0;
}
