#include <stdio.h>
#include <stdlib.h>

#define N 3*100000

int main()
{
	char inStr[N];
	int line_num = 0;
	int check_line = 0;
	char kara;
	int change_num = N, change_index = 0;

	scanf("%d", &line_num);
	scanf("%c", &kara);
	gets(inStr);

	while (inStr[check_line] != 0) {
		check_line++;
	}

	if (check_line != line_num) {
		return 0;
	}

	for (int i = 0; i < line_num; i++) {
		int count = 0;
		int j;
		for (j = 0; j < i; j++) {
			if (inStr[j] == 'W')
				count++;
		}

		for (j = i+1; j < line_num; j++) {
			if (inStr[j] == 'E')
				count++;
		}

		if (change_num > count) {
			change_num = count;
			change_index = i;
		}

	}

	printf("%d\n", change_num);

    return 0;
}

