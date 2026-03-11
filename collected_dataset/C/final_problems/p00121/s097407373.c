#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int d[76543211],f,l,i,j,k;
char Q[50000][9];

void enq(char s[9])
{
	strcpy(Q[l++],s);
}

int main()
{
	int m[4]={1,4,-4,-1};
	char s[9]={48,49,50,51,52,53,54,55,0},c[18],w[9],t;
	memset(d,-1,sizeof(d));
	d[1234567]=0;
	enq(s);
	while(l!=f)
	{
		strcpy(s,Q[f++]);
		strcpy(w,s);
		for(i=0;i<8;i++)
			if(s[i]=='0')
				break;
		if(i==3)
			j=1,k=4;
		else if(i==4)
			j=0,k=3;
		else
			j=0,k=4;
		for(;j<k;j++)
			if(i+m[j]>=0&&i+m[j]<8)
			{
				t=s[i+m[j]];
				s[i+m[j]]=s[i];
				s[i]=t;
				int x = atoi(s);
				int y=atoi(w);
				if(d[x]==-1||d[x]>d[y]+1)
				{
					d[x]=d[y]+1;
					enq(s);
				}
				strcpy(s,w);
			}
	}
	while((c[0]=getchar())!=-1)
	{
		for(i=1;i<16;i++)
			c[i]=getchar();
		for(i=0,j=0;i<8;i++,j+=2)
			s[i]=c[j];
		printf("%d\n",d[atoi(s)]);
	}
	return 0;
}