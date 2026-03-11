#include <stdio.h>

int uruu(int i);

int main(void)
{
	int i,a,b;

	for(scanf("%d%d",&a,&b); !(a==0 && b==0); scanf("%d%d",&a,&b)){
		int flg=0;
		for(i=a; i<=b; i++){
			if(uruu(i)){
				printf("%d\n",i);
				flg=1;
			}
		}
		if(!flg)
			puts("NA");
		puts("");
	}

	return 0;
}

int uruu(int i)
{
	if(i%400==0)
		return 1;
	else if(i%100==0)
		return 0;
	else if(i%4==0)
		return 1;

	return 0;
}