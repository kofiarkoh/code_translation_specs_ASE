// AOJ 2543 Ancient Scrolls
// 2018.2.9 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char s[3][100005];
char ans[100005];
int t[5], dp[4];
int num[100005];
char v[100005][4];
  
void sort1(char *a)
{
	int i, j, k;
	char t;

	for (i = 0; i < 3; i++) {
		k = i;
		for (j = i+1; j < 4; j++) if (a[j] < a[k]) k = j;
		if (k != i) t = a[i], a[i] = a[k], a[k] = t;
	}
}

void sort2(int *a)
{
	int i, j, k;
	int t;

	for (i = 0; i < 2; i++) {
		k = i;
		for (j = i+1; j < 3; j++) if (a[j] < a[k]) k = j;
		if (k != i) t = a[i], a[i] = a[k], a[k] = t;
	}
}

int check()
{
	int i, j, f, ans;
	int r[3];

	for (i = 0; i < 3; i++) if (dp[i] < 0) return 0;
    r[0] = dp[0] - t[3];
	r[1] = dp[1] - t[2];
	r[2] = dp[2] - t[1];

	f = 0;
	for (i = 0; i < 3; i++) if (r[i] < 0) f++;
	if (f >= 2) return 0;
	for (i = 0; i < 3; i++) if (r[i] < 0) {
		for (j = 0; j < 3; j++) if (i != j) r[j] += r[i];
		r[i] = 0;
	}

	f = 0;
	for (i = 0; i < 3; i++) if (r[i] < 0) f++;
	if (f) return 0;
	sort2(r);
	r[0] += r[1];
	ans = (r[0]+r[2]) >> 1;
	if (ans > r[0]) ans = r[0];
	return ans >= t[0];
}  

int main()
{
	int l, d, i, j, k, a;
	char buf[15], *p;

	while (fgets(p=buf, 15, stdin) && *p != '0') {
		l = atoi(p); while (*p >= '0') p++; d = atoi(p+1);
		for (i = 0; i < 3; i++) {
			fgets(s[i], 100005, stdin), dp[i] = d;
		}

		memset(t, 0, sizeof(t));
		for (i = 0; i < l; i++) {
			a = 0;
			for (j = 0; j < 3; j++) for (k = j+1; k < 3; k++) {
				if (s[j][i] == s[k][i]) a += j+k;
			}
			if (a > 4) a = 4;
			num[i] = a, t[a]++;

			v[i][0] = 'A'; for (j = 0; j < 3; j++) v[i][j+1] = s[j][i];  
			sort1(v[i]);
		}

		for (i = 0; i < l; i++) {
			t[num[i]]--;
			for (j = 0; ; j++) {
				if (j == 4) goto ng;
				for (k = 0; k < 3; k++) if (s[k][i] != v[i][j]) dp[k]--;
				if (check()) { ans[i] = v[i][j]; break; }
				for (k = 0; k < 3; k++) if (s[k][i] != v[i][j]) dp[k]++;
			}
		}
		ans[l] = 0;
		puts(ans);
		continue;
ng:		puts("-1");
	}
	return 0;
}
