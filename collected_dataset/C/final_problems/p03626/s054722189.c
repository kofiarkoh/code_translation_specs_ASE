#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <limits.h>
#include <stdbool.h>

typedef double db;
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
	int n;
	in("%d\n",&n);
	if(n==1)
	{
		puts("3");
		return 0;
	}
	lli ret=-1;
	int precol=-1;
	char str[n+1];
	int i;
	gets(str);
	for(i=0;i<n-1;++i)
	{
		if(str[i]==str[i+1]){
			if(precol==0)
			{
				ret *= 3;
			}
			else if (precol == 1)
			{
				ret *= 2;
			}
			else 
			{
				ret = 6;
			}
			precol = 0;
			++i;
		}
		else if(str[i]!=str[i+1])
		{
			if(precol == 0)
			{
				ret *= 1;
			}
			else if(precol==1)
			{
				ret *= 2;
			}
			else
			{
				ret = 3;
			}
			precol = 1;
		}
		ret %= 1000000007;

	}
	if(precol==1) {
		ret *= 2;
		ret %= 1000000007;
	}
	printf("%lld\n",ret);
  return 0;
}