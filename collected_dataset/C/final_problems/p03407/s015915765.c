#include <stdio.h>
 
int main(void)
{
	int A;
	int B;
	int C;
	
	scanf("%d", &A);
	scanf("%d", &B);
	scanf("%d", &C);
	
	if ((A+B)>=C){
		printf("Yes\n");
	}
	else if ((A+B)<=C){
		printf("No\n");
	}
	
	return (0);
}