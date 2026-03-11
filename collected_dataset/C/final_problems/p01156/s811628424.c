// AOJ 2031: Hyper Rock-Scissors-Paper
// 2017.11.04 bal4u@uu

#include <stdio.h>
#include <string.h>

#define HASHSIZ 101
typedef struct { char *s; int id; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash + HASHSIZ;			// Hash table

int lookup(char *s)
{
	int w = strlen(s);
	int i = (101 * *s + 103 * *(s+w-1) + 107 * w) % HASHSIZ;
	HASH *tp = hash + i;

	while (tp->s != NULL) {
		if (!strcmp(tp->s, s)) return tp->id;
		if (++tp == hashend) tp = hash;
	}
	return -1;
}

void insert(char *s, int id)
{
	int w = strlen(s);
	int i = (101 * *s + 103 * *(s+w-1) + 107 * w) % HASHSIZ;
	HASH *tp = hash + i;

	while (tp->s != NULL) {
		if (!strcmp(tp->s, s)) return;
		if (++tp == hashend) tp = hash;
	}
	tp->s = s, tp->id = id;
}

char *hand[15] = {
	"Rock","Fire","Scissors","Snake","Human","Tree","Wolf","Sponge",
	"Paper","Air","Water","Dragon","Devil","Lightning","Gun" };

int f[15][3];		// [0]:members, [1]:win, [2]:defeated

int main()
{
	int n, i, j, ans;
	char buf[12];

	for (i = 0; i < 15; i++) insert(hand[i], i);

	while (scanf("%d", &n) && n > 0) {
		memset(f, 0, sizeof(f));
		for (i = 0; i < n; i++) {
			scanf("%s", buf);
			f[lookup(buf)][0] = 1;
		}

		for (i = 0; i < 15; i++) if (f[i][0]) {
			for (j = 1; j <= 7; j++) {
				if (f[(i+j) % 15][0]) { f[i][1] = 1;  break; }
			}
			for (j = 8; j < 15; j++) {
				if (f[(i+j) % 15][0]) f[i][2] = 1;
			}
		}

		ans = -1;
		for (i = 0; i < 15; i++) if (f[i][1] && !f[i][2]) ans = i;
		if (ans >= 0) puts(hand[ans]);
		else puts("Draw");
	}
	return 0;
}