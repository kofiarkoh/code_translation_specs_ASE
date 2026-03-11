#include<stdio.h>
#include<string.h>
int i,j,c[130]={0},p;
char D[1005],R[1005];

int main()
{
	gets(D);
	strcpy(R,D);
	for(i=0;i<strlen(D);i++)
	{
		if(i>=8&&p>7)c[D[i-8]]--;
		c[D[i]]++;p++;
		if(c['A']==2&&c['I']==1&&c['D']==1&&c['U']==1&&c['N']==2&&c['Y']==1)
		{
			char t=R[i+1];
			strcpy(R+i-7,"AIZUNYAN");
			R[i+1]=t;
			p=0;
			memset(c,0,sizeof(c));
		}
	}
	puts(R);
	return 0;
}