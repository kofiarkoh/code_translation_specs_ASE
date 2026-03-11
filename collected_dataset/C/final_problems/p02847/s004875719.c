#include <stdio.h>
int main(void)
{
	char str[4];
	scanf("%s", str);
	if (str[0] == 'S' && str[1] == 'U') printf("7");
	else if (str[0] == 'M' && str[1] == 'O') printf("6");
	else if (str[0] == 'T' && str[1] == 'U') printf("5");
	else if (str[0] == 'W' && str[1] == 'E') printf("4");
	else if (str[0] == 'T' && str[1] == 'H') printf("3");
	else if (str[0] == 'F' && str[1] == 'R') printf("2");
	else if (str[0] == 'S' && str[1] == 'A') printf("1");
}