#include<stdio.h>
#include<stdbool.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
#define rep(i,n) for(int i=0; i <(n); i++)
int main(){
  long long a,b,v,w,t;
  scanf("%lld%lld%lld%lld%lld",&a,&v,&b,&w,&t);
  if(v==w){
    printf("NO\n");
    return 0;
  }
  if(b>a){
    if(w>v){
      printf("NO\n");
      return 0;
    }
  }
  if((b-a)%(v-w)==0){
    if((b-a)/(v-w)<=t){
      printf("YES\n");
      return 0;
    }else{
      printf("NO\n");
      return 0;
    }
  }
  printf("NO\n");
  return 0;
}