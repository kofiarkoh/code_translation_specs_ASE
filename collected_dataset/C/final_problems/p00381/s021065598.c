#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
#define FOR(n) for(int i=0;i<n;i++)
#define FORJ(n) for(int j=0;j<n;j++)
#define FORL(n) for(long long int i=0;i<n;i++)
#define PRN(n) printf("%d\n",n)
#define PRL(n) printf("%lld\n",n)
#define PRF(f) printf("%.10lf\n",f)
#define PRS(s) printf("%s\n",s)
#define PRC(c) printf("%c",c)
#define mod 1000000007
typedef long long int ll;
ll a[26];
int main(void){
  ll n;
  scanf("%lld",&n);
  char s[n+1],t[n+1];
  scanf("%s %s",s,t);
  a[s[0]-'a']=1;
  for(ll i=1;i<n-1;i++){
    a[s[i]-'a']+=a[t[i]-'a'];
    a[s[i]-'a']%=mod;
  }
  PRL(a[t[n-1]-'a']);
  return 0;
}

