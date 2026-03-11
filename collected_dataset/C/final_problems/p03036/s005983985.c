#include<stdio.h>
int main(){
  int r,d,x;
  scanf("%d%d%d",&r,&d,&x);
  long long int ans=x;
  for(int i=0;i<10;i++){
    ans=ans*r-d;
    printf("%lld\n",ans);
  }
}