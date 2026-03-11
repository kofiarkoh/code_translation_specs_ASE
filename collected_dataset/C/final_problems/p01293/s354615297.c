// AOJ 2175: Whist
// 2017.11.5 bal4u@uu

#include <stdio.h>

int c[4][13][2];
char tr['Z'];

int main()
{
	int i, j, suit, rank, leader, max, winner;
	char buf[5], score[2], trump;

	tr['A'] = 14, tr['T'] = 10, tr['J'] = 11, tr['Q'] = 12, tr['K'] = 13;
	for (i = 2; i <= 9; i++) tr['0'+i] = i;

	while (scanf("%s", buf) && *buf != '#') {
		trump = *buf;
		for (i = 0; i < 4; i++) for (j = 0; j < 13; j++) {
			scanf("%s", buf);
			c[i][j][0] = tr[*buf], c[i][j][1] = *(buf+1);
		}

		score[0] = score[1] = 0;
		leader = c[0][0][1];
		for (j = 0; j < 13; j++) {
			winner = max = -1;
			for (i = 0; i < 4; i++) {
				rank = c[i][j][0], suit = c[i][j][1];
				if (suit != trump && suit != leader) continue;
				if (suit == trump) rank += 100;
				if (max < rank) max = rank, winner = i;
			}
			score[winner & 1]++;
			leader = c[winner][j+1][1];
		}
		if (score[0] > score[1]) printf("NS %d\n", score[0]-6);
		else                     printf("EW %d\n", score[1]-6);
	}
	return 0;
}