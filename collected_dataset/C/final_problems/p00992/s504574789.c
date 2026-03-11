#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
int b[10000010]={0};
int main(){
  int n,a,ans,i;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d",&a);
    b[a]++;
  }
  for(i=ans=0;n;i++){
    ans+=MIN(n,i*4);
    n-=b[i];
  }
  printf("%d\n",ans+1);
}