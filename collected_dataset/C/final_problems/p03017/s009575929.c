#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>
#pragma warning( disable:4996 )
#define SWAP(a,b) (a += b,b = a - b,a -= b)
#define MIN(a, b) ((a) < (b) ? (a) : (b))


int main(void) {
	int n, a, b, c, d;
	char s[200002];
	scanf("%d %d %d %d %d", &n, &a, &b, &c, &d);
	scanf("%s", s);

	if (c < d) {
		while (1) {
			if (d - b <= 2) {
				b = d;
				break;
			}
			if (s[b + 1] == '.') {
				b++;
				continue;
			}
			if (s[b + 2] == '.') {
				b += 2;
				continue;
			}
			printf("No\n");
			return 0;
		}
		while (1) {
			if (c - a <= 2) {
				a = c;
				break;
			}
			if (s[a + 1] == '.') {
				a++;
				continue;
			}
			if (s[a + 2] == '.') {
				a += 2;
				continue;
			}
			printf("No\n");
			return 0;
		}
		printf("Yes\n");
	}else{
		int spaceok = 0;
		int index;
		for (int i = b; i <= d; i++) {
			if (s[i - 2] == '.' && s[i - 1] == '.'&&s[i] == '.') {
				spaceok = 1;
				index = i;
				break;
			}
		}
		if (spaceok) {
			//move b
			while (1) {
				if (index - b <= 2) {
					b = index;
					break;
				}
				if (s[b + 1] == '.') {
					b++;
					continue;
				}
				if (s[b + 2] == '.') {
					b += 2;
					continue;
				}
				printf("No\n");
				return 0;
			}
			while (1) {
				if (c - a <= 2) {
					a = c;
					break;
				}
				if (s[a + 1] == '.') {
					a++;
					continue;
				}
				if (s[a + 2] == '.') {
					a += 2;
					continue;
				}
				printf("No\n");
				return 0;
			}
			while (1) {
				if (d - b <= 2) {
					printf("Yes\n");
					return 0;
					break;
				}
				if (s[b + 1] == '.') {
					b++;
					continue;
				}
				if (s[b + 2] == '.') {
					b += 2;
					continue;
				}
				printf("No\n");
				return 0;
			}
		}
		else {
			printf("No\n");
			return 0;
		}
	}

	return 0;
}