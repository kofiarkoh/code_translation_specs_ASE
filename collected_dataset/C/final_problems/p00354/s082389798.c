#include<stdio.h>
int main()
{
	int X,day;
	scanf("%d",&X);
	day=X%7;
	if(day==0)
	puts("thu");
	else if(day==1)
	puts("fri");
	else if(day==2)
	puts("sat");
	else if(day==3)
	puts("sun");
	else if(day==4)
	puts("mon");
	else if(day==5)
	puts("tue");
	else if(day==6)
	puts("wed");
	return 0;
}
