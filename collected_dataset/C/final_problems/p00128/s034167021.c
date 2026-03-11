#include <stdio.h>          // putchar(), scanf()
#include <stdbool.h>

int
main(int argc, char **argv)
{
	char abacus[10][8] = {"* = ****",  "* =* ***",  "* =** **",  "* =*** *",  "* =**** ",  " *= ****",  " *=* ***",  " *=** **",  " *=*** *",  " *=**** "};
	int x;

	bool f = false;
	while (scanf("%d", &x) != EOF)
	{
		int a[5];
		for (int i = 4; i >= 0; --i)
		{
			a[i] = x % 10;
			x /= 10;
		}

		if (f)
			putchar('\n');

		for (int i = 0; i < 8; ++i)
		{
			for (int j = 0; j < 5; ++j)
				putchar(abacus[a[j]][i]);

			putchar('\n');
		}

		f = true;
	}

	return 0;
}