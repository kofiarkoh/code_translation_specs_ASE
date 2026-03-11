//C - Marks
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
//#define N 998244353
#define pi 3.1415926535897932384626433832

int gcd(int m,int n){if((0==m)||(0==n))return 0;while(m!=n)if(m>n)m=m-n;else n=n-m;return m;}
int lcm(int m,int n){if((0==m)||(0==n))return 0;return(m*n/gcd(m, n));}
int fact(int x){if(x==1) return(x);else return(x*fact(x-1));}//Kaijou x!
int ruijou(int base,int exponent){int i,answer=1;i=exponent;while(i!=0){answer=answer*base;--i;}return answer;}//ruijou base^exponent

long long int llgcd(long long int m,long long int n){if((0==m)||(0==n))return 0;while(m!=n)if(m>n)m=m-n;else n=n-m;return m;}
long long int lllcm(long long int m,long long int n){if((0==m)||(0==n))return 0;return(m*n/llgcd(m, n));}
long long int llfact(long long int x){if(x==1) return(x);else return(x*llfact(x-1));}//Kaijou
int llruijou(long long int base,long long int exponent){long long int i,answer=1;i=exponent;while(i!=0){answer=answer*base;--i;}return answer;}//ruijou base^exponent

int main()
{
  ll n,k;
  scanf("%lld %lld", &n,&k);

  ll a[200010]={},b[200010]={};
  for(ll i = 1; i <= n; i++){
    scanf("%lld", &a[i]);
  }
  ll num=1;
  for(ll i = 1; i <= k; i++){
    num*=a[i];
  }
  b[k]=num;
  for(ll i = k+1; i <= n; i++){
    b[i]=b[i-1]/a[i-k]*a[i];
  }
  for(ll i = k+1; i <= n; i++){
    if(b[i]>b[i-1]){
      printf("Yes\n");
    }
    else{
      printf("No\n");
    }
  }

  return 0;
}
