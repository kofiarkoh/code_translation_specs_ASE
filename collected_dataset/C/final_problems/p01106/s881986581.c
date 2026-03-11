#include<stdio.h>
int main(){
 long long n,a,b;
 char s[100];
 while(scanf("%lld %lld %lld",&n,&a,&b),n){
  a--;
  b--;
  s[n]=0;
  while(n--){
   s[n]=(a%2==b%2)?'L':'R';
   a=(a<(1LL<<n))?(1LL<<n)-a-1:a-(1LL<<n);
   b/=2;
  }
  printf("%s\n",s);
 }
 return 0;
}
