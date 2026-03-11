#include<stdio.h>
#define ll long long
char S[50];
int p,C[30]={0},f=0,i;
ll r=0,b=1;

ll fact(int i)
{
	ll res=1;
	for(;i;i--)
		res*=i;
	return res;
}

int main()
{
	gets(S);
	for(i=0;S[i];i++)
	{
		C[S[i]-'a']++;
	}
	for(i=0;i<=27;i++)
	{
		if(C[i]%2)
		{
			if(f)
			{
				puts("0");
				return 0;
			}
			f=1;
		}
		r+=C[i]/2;
		b*=fact(C[i]/2);
	}
	printf("%lld\n",fact(r)/b);
	return 0;
}