// AOJ Vol30-3039 Special Chat
// 2019.1.12

//#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>

int main()
{
	int P;

	scanf("%d", &P);
	printf("%d\n", P - P % 500);
	return 0;
}
