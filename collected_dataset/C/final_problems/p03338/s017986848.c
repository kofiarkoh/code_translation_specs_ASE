#include<stdio.h>
#include<math.h>
int main()
{
	char str[200];
	int n,a[200]={0},i,j,max=0,x,y,ok,t;
	scanf("%d",&n);
	scanf("%s",str);
	for(i=0;i<n;i++)
	{
		ok=1;
		if(str[i]=='0')
			continue;
		for(j=n-1;j>i;j--)
		{
			if(str[j]==str[i] && ok==1)
			{
				for(x=i;x<=j;x++) a[x]++;
				ok=0;
			}
			else if(ok==0 && str[j]==str[i])
				str[j]='0';
		}
		if(i==j && ok==1)
			str[i]='0';
	}
	for(i=0;i<n;i++)
		max=max>a[i]?max:a[i];
	printf("%d\n",max);
	return 0;
}