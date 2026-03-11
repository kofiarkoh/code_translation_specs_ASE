#include<stdio.h>
int main()
{
	int a[12] = {1,3,5,7,8,10,12,4,6,9,11,2};
	int x,y,i,b,c;
	scanf("%d %d",&x,&y);
	for(i = 0;i < 12;i++)
	{
		if(x == a[i])
			b = i;
		if(y == a[i])
			c = i;
	}
	if((b >= 0 && b <= 6) && (c >= 0 && c <= 6))
		printf("Yes\n");
	else if((b >= 7 && b <= 10) && (c >= 7 && c <= 6))
		printf("Yes\n");
	else
		printf("No\n");
	return 0;
}
