#include <stdio.h>

typedef struct {
	int is_left;
	int pos;
	int alive;
} ant_t;

int solve(void) {
	int n, l;
	ant_t ants[30];
	int i;
	int e_time = 0;
	int last_ant = 0;
	if (scanf("%d%d", &n, &l) != 2 || (n == 0 && l == 0)) return 0;
	for (i = 0; i < n; i++) {
		char d[4];
		if (scanf("%s%d", d, &ants[i].pos) != 2) return 0;
		ants[i].is_left = (d[0] == 'L');
		ants[i].alive = 1;
	}
	for (;;) {
		int ant_lives = 0;
		int ant_count[110] = {};
		for (i = 0; i < n; i++) {
			if (ants[i].alive) {
				ant_lives = 1;
				if (ants[i].is_left) {
					ants[i].pos--;
				} else {
					ants[i].pos++;
				}
				ant_count[ants[i].pos]++;
			}
		}
		if (!ant_lives) break;
		e_time++;
		for (i = 0; i < n; i++) {
			if (ants[i].alive && ants[i].pos == l) {
				ants[i].alive = 0;
				last_ant = i;
			}
		}
		for (i = 0; i < n; i++) {
			if (ants[i].alive) {
				if (ants[i].pos == 0) {
					ants[i].alive = 0;
					last_ant = i;
				} else if(ant_count[ants[i].pos] > 1) {
					ants[i].is_left = !ants[i].is_left;
				}
			}
		}
	}
	printf("%d %d\n", e_time, last_ant + 1);
	return 1;
}

int main(void) {
	while(solve());
	return 0;
}