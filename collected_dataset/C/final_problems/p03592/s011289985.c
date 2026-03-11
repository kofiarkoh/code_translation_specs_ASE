#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <limits.h>
#include <stdbool.h>

typedef double db;
typedef int in;
typedef long int li;
typedef long long int lli;
typedef long ld;
typedef long lld;

#define pi M_PI
#define in scanf
#define out printf
#define inf INT_MAX
#define rep(i,n) for(int i=0;i<n;++i)
#define debugd(d) printf("%d\n",d)
#define debugs(s) printf("%s\n",s)
#define debugc(c) printf("%c\n",c)
#define debugf(f) printf("%f\n",f)


int main (void)
{
  int n,m,k;
  scanf("%d %d %d \n",&n,&m,&k);
  rep(i,n){
    rep(j,m){
      if (n*i+j*m-2*i*j==k) {
        puts("Yes");
        return 0;
      }  
    }
  }
  puts("No");
  return 0;
}