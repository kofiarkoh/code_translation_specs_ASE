#include <stdio.h>
int main(){
	int a, b, c, x, i = 0;
	scanf("%d%d%d", &a, &b, &c);
	
	if (c<a)
	{ }
	else if (c < b){
		if (c%a == 0)
			i++;
		for (x = a + 1; x <= c; x++)
		{
			if (c%x == 0)
			{
				i++;
			}
		}
	}
	else{
		if (c%a == 0)
			i++;
		for (x = a + 1; x <= b; x++)
		{
			if (c%x == 0)
			{
				i++;
			}
		}
	}
	printf("%d\n", i);
	return 0;
}