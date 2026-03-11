#include<stdio.h>

int main(){
  long long a[100000] = {};
  long long p[100000] = {};
  long long n = 0;
  long long i = 0;
  long long cnt = 1;
  long long ck = 0;
  scanf("%lld",&n);
  for(i=0;i<n;i++){
    scanf("%lld",&a[i]);
    if(a[i] == 0){
      ck++;
    }
  }
  for(i=0;i<n;i++){
    if(a[i] != 0){
      if((3-p[a[i]]) >= 0){
        cnt = (cnt*(p[a[i]-1]-p[a[i]]))%1000000007;
      }else{
        cnt = 0;
      }
    }
    //printf("%lld/",cnt);
    p[a[i]]++;
  }
  if(ck == 1){
    cnt = (cnt * 3)%1000000007;
  }else if(ck == 2){
    cnt = (cnt * 6)%1000000007;
  }else if(ck == 3){
    cnt = (cnt * 6)%1000000007;
  }else{
    cnt = 0;
  }
  printf("%lld",cnt);

  return 0;
}