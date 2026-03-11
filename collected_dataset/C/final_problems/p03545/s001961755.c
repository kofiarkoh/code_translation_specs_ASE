#include<stdio.h>
int main()
{   
    char a[4];
    int x,y,z,p;
	scanf("%s",&a);
	x=a[0]-'0';
	y=a[1]-'0';
	z=a[2]-'0';
	p=a[3]-'0';
	if(x+y+z+p==7)
	{
		printf("%d+%d+%d+%d=7\n",x,y,z,p);
		return 0;
	}
	if(x+y+z-p==7)
	{
		printf("%d+%d+%d-%d=7\n",x,y,z,p);
		return 0;
	}
	if(x+y-z+p==7)
	{
		printf("%d+%d-%d+%d=7\n",x,y,z,p);
		return 0;
	}
	if(x-y+z+p==7)
	{
		printf("%d-%d+%d+%d=7\n",x,y,z,p);
		return 0;
	}
	if(x+y-z-p==7)
	{
		printf("%d+%d-%d-%d=7\n",x,y,z,p);
		return 0;
	}
	if(x-y+z-p==7)
	{
		printf("%d-%d+%d-%d=7\n",x,y,z,p);
		return 0;
	}
	if(x-y-z+p==7)
	{
		printf("%d-%d-%d+%d=7\n",x,y,z,p);
		return 0;
	}
	if(x-y-z-p==7)
	{
		printf("%d-%d-%d-%d=7\n",x,y,z,p); 
		return 0;
	}	
}
