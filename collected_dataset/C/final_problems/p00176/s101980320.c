#include<stdio.h>
char a[8][8]={"black","blue","lime","aqua","red","fuchsia","yellow","white"},c;
int b[9][3]={{0,0,0},{0,0,255},{0,255,0},{0,255,255},{255,0,0},{255,0,255},{255,255,0},{255,255,255}};

int d(int x)
{
	int i,r=0;
	for(i=0;i<3;i++)
		r+=(b[8][i]-b[x][i])*(b[8][i]-b[x][i]);
	return r;
}

int main()
{
	int i,r;
	for(;~scanf("%c%2x%2x%2x\n",&c,&b[8][0],&b[8][1],&b[8][2]),c!='0';puts(a[r]))
	{
		for(i=r=0;i<8;i++)
			if(d(r)>d(i))
				r=i;
	}
	return 0;
}