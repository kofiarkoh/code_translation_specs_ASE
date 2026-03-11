#include<stdio.h>

int main(void)
{
	char class[20];
	int i, am, pm;
	
	for(i=0;i<9;i++)
	{
		scanf("%s %d %d",class,&am,&pm);
		printf("%s %d %d\n",class,am+pm,(200*am)+(300*pm));
	}
	return 0;
}