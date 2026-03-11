#include <stdio.h>

int main()
{
	int A, B, C;
	scanf("%d %d %d", &A, &B, &C);
	int r;
	if(B/A > C){
		r = C;
	} else {
		r = B/A;
	}
	printf("%d\n",r);
	return 0;
}
