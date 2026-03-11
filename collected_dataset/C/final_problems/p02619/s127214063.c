#include <stdio.h>

int main(){
  int D;
  int c[30];
  int s[370][30];
  int t[400];
  int last[30]={};
  int ans=0;
  
  scanf("%d",&D);
  for(int i=1;i<=26;i++)
    last[i]=0;
  for(int i=1;i<=26;i++)
    scanf("%d",c+i);
  for(int i=1;i<=D;i++)
   for(int j=1;j<=26;j++)
     scanf("%d",&s[i][j]);
  for(int i=1;i<=D;i++)
    scanf("%d",t+i);
  for(int d=1;d<=D;d++){
    ans+=s[d][t[d]];
    last[t[d]]=d;
    for(int j=1;j<=26;j++){
      ans-=c[j]*(d-last[j]);
    }
    printf("%d\n",ans);
  }
  
  return 0;
}