#include <stdio.h>

int main()
{
	int i, N;
	char s[100001];
	scanf("%d", &N);
	scanf("%s", s);
	
	int j;
	char t[4][100001] = {"SS", "ST", "TS", "TT"};
	for (i = 1; i < N - 1; i++) {
		for (j = 0; j < 4; j++) {
			if (s[i] == 'o') {
				if (t[j][i] == 'S') t[j][i+1] = t[j][i-1];
				else if (t[j][i-1] == 'S') t[j][i+1] = 'W';
				else t[j][i+1] = 'S';
			} else if (t[j][i] == 'W') t[j][i+1] = t[j][i-1];
			else if (t[j][i-1] == 'S') t[j][i+1] = 'W';
			else t[j][i+1] = 'S';
		}
	}
	
	for (j = 0; j < 4; j++) {
		t[j][N] = 0;
		if (s[0] == 'o' && t[j][1] != t[j][N-1]) continue;
		else if (s[0] == 'x' && t[j][1] == t[j][N-1]) continue;
		else if (s[N-1] == 'o' && t[j][N-1] == 'S' && t[j][0] != t[j][N-2]) continue;
		else if (s[N-1] == 'o' && t[j][N-1] == 'W' && t[j][0] == t[j][N-2]) continue;
		else if (s[N-1] == 'x' && t[j][N-1] == 'S' && t[j][0] == t[j][N-2]) continue;
		else if (s[N-1] == 'x' && t[j][N-1] == 'W' && t[j][0] != t[j][N-2]) continue;
		else break;
	}
	if (j < 4) printf("%s\n", t[j]);
	else printf("-1\n");
	fflush(stdout);
	return 0;
}