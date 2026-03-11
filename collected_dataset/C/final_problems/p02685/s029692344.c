#include <stdio.h>
long powmod(long a,long b,long m){
    if(b==0){return 1;}
    long c;
    c = powmod((a*a)%m,b/2,m);
    if(b%2){
        c=(c*a)%m;
    }
    return c;
}
int main(void){
  long n,m,k;
  long i;
  scanf("%ld %ld %ld", &n, &m, &k);
  long mod = 998244353;
  long sum = (m*powmod(m-1,n-1,mod))%mod;
  long c=1;
  for(i=1;i<=k;i++){
    c = (c *(((n-i)*powmod(i,mod-2,mod))%mod))%mod;
    sum = (sum + ((m*c)%mod)*powmod(m-1,n-1-i,mod))%mod;
  }
  printf("%ld",sum);
}
