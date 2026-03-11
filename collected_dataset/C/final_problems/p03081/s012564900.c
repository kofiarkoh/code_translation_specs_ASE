#include <stdio.h>

int main()
{	
	int i, N, Q;
	char s[200001], t[200001], d[200001];
	scanf("%d %d", &N, &Q);
	scanf("%s\n", s);
	for (i = 0; i < Q; i++) scanf("%c %c\n", &(t[i]), &(d[i]));
	
	int l = -1, r = N;
	for (i = Q - 1; i >= 0 && l < r - 1; i--) {
		if (d[i] == 'L') {
			if (s[l+1] == t[i]) l++;
			if (r < N && s[r] == t[i]) r++;
		} else {
			if (l >= 0 && s[l] == t[i]) l--;
			if (s[r-1] == t[i]) r--;
		}
	}
	
	printf("%d\n", r - l - 1);
	fflush(stdout);
	return 0;
}