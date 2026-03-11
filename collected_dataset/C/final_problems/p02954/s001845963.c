#include<stdio.h>
#include<string.h>
#include<math.h>

typedef struct {
	int even;
	int odd;
}child_t;

int main(void) {
	char s[100000] = { 0 };
	child_t child[100000] = { 0 };
	int temp,ans;

	scanf("%s", s);
	int len = strlen(s);
	for (int i = 0; i < len; i++) {
		if (s[i] == 'R') {
			for (int j = 1; j < len; j++) {
				if (s[i + j] == 'L') {
					temp = j;
					break;
				}
			}
			if (temp % 2) {
				child[i].odd = i + temp;
				child[i].even = i + temp-1;
			}
			else {
				child[i].even = i + temp;
				child[i].odd = i + temp - 1;
			}
		}
		else {
			for (int j = 1; j < len; j++) {
				if (s[i - j] == 'R') {
					temp = j;
					break;
				}
			}
			if (temp % 2) {
				child[i].odd = i - temp;
				child[i].even = i - temp + 1;
			}
			else {
				child[i].even = i - temp;
				child[i].odd = i - temp + 1;
			}
		}
	}
	for (int i = 0; i < len; i++) {
		ans = 0;
		for (int j = 0; j < len; j++) {
			if (i == child[j].even) ans++;
		}
		printf("%d ", ans);
	}
	return 0;
}
