#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define max(a,b)(a>b?a:b)
char S[80005];
int i,j,k,l,r,I[80005],C[80005][2],p,q;

int set(int p,int n)
{
	int i;
	for(i=0;i<q;i++)
		if(C[i][0]==p)
		{
			C[i][1]+=n;
			return C[i][1];
		}
	C[q][0]=p;
	C[q][1]=n;
	q++;
	return n;
}

int cnt(int p)
{
	int i,r=0;
	for(i=0;i<q;i++)
		if((p-C[i][0])%11==0)
			r+=C[i][1];
	return r;
}

int main()
{
	for(;gets(S),S[0]!='0';)
	{
		r=p=q=0;
		memset(C,-1,sizeof(C));
		l=strlen(S);
		for(i=0,j=l-1;i<=j;i++,j--){k=S[i]-'0';S[i]=S[j]-'0';S[j]=k;}
		set(0,1);
		for(i=1,k=1;i<=l;i++,k*=-1)
		{
			I[i]=I[i-1]+S[i-1]*k;
			if(S[i-1]!=0)
				r+=cnt(I[i]);
			set(I[i],1);
			
		}
		printf("%d\n",r);
	}
	return 0;
}