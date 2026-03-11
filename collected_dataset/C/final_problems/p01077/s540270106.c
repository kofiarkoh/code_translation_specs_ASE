// AOJ 1592: Curling Puzzle
// 2017.11.29 bal4u@uu

#include <stdio.h>

#define MIN(a,b) ((a)<=(b)?(a):(b))

int sr[17], sc[17], sz;
int gr, gc;

int main() 
{
	int H, W, r, c, ans;
	char m[20];

	scanf("%d%d", &H, &W);

	ans = 1;
	for (r = 0; r < H; r++) {
		scanf("%s", m);
		for (c = 0; c < W; c++) {
			if (m[c] == 'o') {
				sr[sz] = r, sc[sz] = c;
				if (++sz >= 16) goto done;
			} else if (m[c] == '@') gr = r, gc = c;
		}
	}
	if (H == 1) { ans = (sz > MIN(gc, W-1-gc)); goto done; }
	if (W == 1) { ans = (sz > MIN(gr, H-1-gr)); goto done; }
	if (sz >= 3) goto done;

	if (sz >= 1) {
		ans = (!MIN(gr, H-1-gr) && !MIN(gc, W-1-gc) ||
			   !MIN(gr, H-1-gr) && gc == sc[0] ||
			   !MIN(gc, W-1-gc) && gr == sr[0]);
		if (ans || sz == 1)	goto done;
	}

	ans = (MIN(gr, H-1-gr) < 2 || MIN(gc, W-1-gc) < 2); 

done:
	puts(ans ? "yes" : "no");
	return 0; 
}