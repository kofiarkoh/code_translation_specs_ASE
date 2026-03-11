#include<stdio.h>

int N,i,j,c,a,b;
char D[20];
int cv(char c)
{
	if(c<';')return c-'0';
	else if(c=='A'){a++;return 11;}
	else return 10;
}
int main()
{
	scanf("%d\n",&N);
	for(;N--;)
	{
		for(i=0;i<10;i++)
			scanf("%c%*c",D+i);
		for(i=1;;i++)
		{
			for(j=a=c=0;j<=i;j++)
				c+=cv(D[j]);
			for(b=c;b>21&&a--;)b-=10;
			if(b>17||(b==17&&!a))break;
		}
		if(b==21&&i==1)puts("blackjack");
		else if(b>21)puts("bust");
		else printf("%d\n",b);
	}
	return 0;
}