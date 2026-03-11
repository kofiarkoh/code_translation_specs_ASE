#include<stdio.h>

int main()
{
	int A, B, C;
	scanf("%d %d %d", &A, &B, &C);

	if((A == B && B != C) || (B == C && B != A) || (A == C && C != B))
	{printf("Yes\n");}

	else
	{printf("No\n");}
}