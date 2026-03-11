#include <stdio.h>

int main()
{
	int H, W, N, sr, sc;
	char S[200001], T[200001];
	scanf("%d %d %d", &H, &W, &N);
	scanf("%d %d", &sr, &sc);
	scanf("%s", S);
	scanf("%s", T);
	
	int i, l = 0, r = W + 1, t = 0, b = H + 1;
	for (i = N - 1; i >= 0; i--) {
		if (T[i] == 'R' && l > 0) l--;
		else if (T[i] == 'L' && r < W + 1) r++;
		else if (T[i] == 'D' && t > 0) t--;
		else if (T[i] == 'U' && b < H + 1) b++;
		if (S[i] == 'L') l++;
		else if (S[i] == 'R') r--;
		else if (S[i] == 'U') t++;
		else if (S[i] == 'D') b--;
		if (r == l + 1 || b == t + 1) break;
	}
	
	if (sc <= l || sc >= r || sr <= t || sr >= b) printf("NO\n");
	else printf("YES\n");
	fflush(stdout);
	return 0;
}