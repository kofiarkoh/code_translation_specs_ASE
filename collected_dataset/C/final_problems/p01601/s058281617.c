#include<stdio.h>
#include<string.h>
int N,c=0;
int ch(int n)
{
	int l,r;
	char S[5];
	sprintf(S,"%d",n);
	for(l=0,r=strlen(S)-1;l<r;l++,r--)
		if(S[l]!=S[r])
			return 0;
	return 1;
}
int main()
{
	scanf("%d",&N);
	for(;;c++)
	{
		if(ch(N-c)){N-=c;break;};
		if(ch(N+c)){N+=c;break;};
	}
	printf("%d\n",N);
	return 0;
}