#include <stdio.h>

void drop_ink();
void taint_1();
void taint_2();
void taint_3();
int untaint();
int mosttaint();

int map[10][10] = {}, in[3];

int main()
{
	while (scanf("%d,%d,%d", &in[0], &in[1], &in[2]) != EOF) {
		drop_ink();
	}
	printf("%d\n%d\n", untaint(), mosttaint());
}

void drop_ink()
{
	switch (in[2]) {
	case 3:
		taint_3();
	case 2:
		taint_2();
	case 1:
		taint_1();
	}
}

void taint_1()
{
	if (in[0] - 1 >= 0) {
		map[in[0] - 1][in[1]]++;
	}
	if (in[0] + 1 < 10) {
		map[in[0] + 1][in[1]]++;
	}
	if (in[1] - 1 >= 0) {
		map[in[0]][in[1] - 1]++;
	}
	if (in[1] + 1 < 10) {
		map[in[0]][in[1] + 1]++;
	}
}
void taint_2()
{
	if (in[0] - 1 >= 0) {
		if (in[1] - 1 >= 0) {
			map[in[0] - 1][in[1] - 1]++;
		}
		if (in[1] + 1 < 10) {
			map[in[0] - 1][in[1] + 1]++;
		}
	}
	if (in[0] + 1 < 10) {
		if (in[1] - 1 >= 0) {
			map[in[0] + 1][in[1] - 1]++;
		}
		if (in[1] + 1 < 10) {
			map[in[0] + 1][in[1] + 1]++;
		}
	}
}
void taint_3()
{
	if (in[0] - 2 >= 0) {
		map[in[0] - 2][in[1]]++;
	}
	if (in[0] + 2 < 10) {
		map[in[0] + 2][in[1]]++;
	}
	if (in[1] - 2 >= 0) {
		map[in[0]][in[1] - 2]++;
	}
	if (in[1] + 2 < 10) {
		map[in[0]][in[1] + 2]++;
	}
}

int untaint()
{
	int p, q, ret = 0;
	for (p = 0; p < 10; p++) {
		for (q = 0; q < 10; q++) {
			if (map[p][q] == 0) {
				ret++;
			}
		}
	}
	return ret;
}

int mosttaint()
{
	int p, q, ret = 0;
	for (p = 0; p < 10; p++) {
		for (q = 0; q < 10; q++) {
			if (map[p][q] > ret) {
				ret = map[p][q];
			}
		}
	}
	return ret;
}