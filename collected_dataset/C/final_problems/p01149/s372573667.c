// AOJ 2024: Blackjack
// 2017.11.04 bal4u@uu

#include <stdio.h>

int score[10];
int tr['Z'];

int main()
{
	int cno, i, s, ace;
	char buf[3];

	tr['A'] = 1, tr['T'] = tr['J'] = tr['Q'] = tr['K'] = 10;
	for (i = 2; i <= 9; i++) tr[i+'0'] = i;

	scanf("%d", &cno); while (cno--) {
		for (i = 0; i < 10; i++) scanf("%s", buf), score[i] = tr[*buf];
		ace = 0;
		for (s = 0, i = 0; ; i++) {
			if (i == 10) { printf("%d\n", s); break; }
			if (s < 17 || s == 17 && ace) {
				if (score[i] == 1 && s + 11 <= 21) s += 11, ace = 1;
				else s += score[i];
				if (s > 21 && ace) s -= 10, ace = 0;
				if (i == 1 && s == 21) { puts("blackjack"); break; }
				if (s > 21) { puts("bust"); break; }
			}
		}
	}
	return 0;
}