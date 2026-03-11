// AOJ 3064 AOJ50M
// 2019.9.30 bal4u

#include <stdio.h>

int main()
{
	int T1, T2, R1, R2, win;
	char *s[] = {"Alice", "Draw", "Bob"};
	
	scanf("%d%d%d%d", &T1, &T2, &R1, &R2);
	if (R1 < 0 || R2 < 0) win = T1 - T2;
	else win = R2 - R1;
	if (win < 0) win = -1;
	else if (win > 0) win = 1;
	puts(s[win+1]);
	return 0;
}
