#include <stdio.h>

int main()
{
	char S[10], T[10], U[10];
	int a, b;
	
	
	scanf("%s %s", S, T);
	
	scanf("%d %d", &a, &b);
	
	scanf("%s", U);
	
	if(*S==*U){ printf("%d %d", a-1, b); }
	else if(*T==*U){ printf("%d %d", a, b-1); }
	
	return 0;
}