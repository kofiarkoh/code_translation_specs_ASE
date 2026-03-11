//C - Product Modulo
//Hiia
#include <stdio.h>
#include <stdlib.h>//rannsuu, zettaichi(absolute)
#include <time.h>
//srand((unsigned int) time(0));
#include <math.h>//koudona keisann
#include <string.h>//hairetsu
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))
typedef long long ll;
#define PP 200003

int main()
{
  ll n,x[200010],i,ans=0;
  scanf("%lld", &n);
  for(ll i = 1; i <= n; i++) {
    scanf("%lld", &x[i]);
  }

  for(ll i = 1; i <= n-1; i++) {
    for(ll j = i+1; j <= n; j++) {
      ans+=(x[i]*x[j]+PP)%PP;
    }
  }

  printf("%lld\n", ans);
  return 0;
}
