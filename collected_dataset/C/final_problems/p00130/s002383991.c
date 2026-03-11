#include<stdio.h>
#include<string.h>
int n,i,j,p;
char c,t,l[27];
int main()
{
	scanf("%d",&n);
	getchar();
	for(i=0;i<n;i++)
	{
		p=13;
		memset(l,0,27);
		for(;scanf("%c",&l[p])!=-1;)
		{
			~scanf("%c",&c);
			if(c=='-'||c=='<')
				getchar(),c=='-'?p++:p--;
			else
				break;
		}
		for(j=0;!l[j];j++);
		puts(l+j);
	}
	return 0;
}