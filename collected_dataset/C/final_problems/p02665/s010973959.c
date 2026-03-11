#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
#define ll long long

int n;
ll a[100010];

int main(){
  scanf("%d",&n);
  for(int i=0;i<=n;i++){
    scanf("%lld",&a[i]);}
  
  ll fflag[100010]={};
  ll ans =0;
  ll count = 1;
  ll tmp;

  ll sum=0;
  for(int i=0;i<=n;i++){
    sum += a[i];}
  if(sum==0){
    printf("0\n");
    return 0;}

  for(int i=0;i<=n;i++){
    if(a[i]>count){
      printf("-1\n");
      return 0;}
    if(sum > count){
      tmp = count;}
    else{
      tmp = sum;}
    ans += tmp;
    sum -= a[i];
    count = (tmp-a[i])*2;}
    
  
  printf("%lld\n",ans);
  
  return 0;
}
