#include<stdio.h>
int main(){
  int n,d[5010],i,j;
  scanf("%d",&n);
  for(i=0;i<n;i++)scanf("%d",&d[i]);
  for(i=0;i<n;i++){
    int ans=0,a[5010]={};
    for(j=i+1;j--;){//printf("%d ",j);
      if(a[j+1])continue;
      ans+=1-a[d[j]];
      a[d[j]]=1;
    }
    printf("%d\n",ans);
  }
  return 0;
}