#define _CRT_SECURE_NO_WARNINGS = false	
#include <stdlib.h>;
#include <stdio.h>;
//#include "Main.h"
int main(void)
{
	// Your code here!
	char S[4];
	char T[4];
	int ans=0;

	scanf("%c%c%c\n", &S[0], &S[1], &S[2]);
	scanf("%c%c%c", &T[0], &T[1], &T[2]);
	for (int i = 0; i<3; i++)
	{
		//printf("%c", S[i]);
		//printf("%c\n", T[i]);
		if (S[i] == T[i])
		{
			ans = ans + 1;
		}
	}
	printf("%d", ans);
	//system("pause");
}
