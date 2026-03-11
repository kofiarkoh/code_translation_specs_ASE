#include <stdio.h>

#define MAX 100000

int N;
int a[MAX];

int logdata[MAX];
int visited[MAX];
int result[MAX];

int main(void) {
	int i, j;
	int count = 0;
	if (scanf("%d", &N) != 1) return 1;
	for (i = 0; i < N; i++) {
		if (scanf("%d", &a[i]) != 1) return 1;
	}
	for (i = 0; i < N; i++) {
		int current = i;
		int log_entry = 0;
		while (result[current] == 0 && visited[current] != i + 1) {
			visited[current] = i + 1;
			logdata[log_entry++] = current;
			current = (current + a[current]) % N;
		}
		if (result[current] != 0) {
			/* loop ni iku node agari niwa naranai */
			for (j = 0; j < log_entry; j++) {
				result[logdata[j]] = -1;
			}
		} else {
			/* loop ni iku made no yatu wa agari ni naranai */
			for (j = 0; logdata[j] != current; j++) {
				result[logdata[j]] = -1;
			}
			/* loop ni haitteiru yatu wa agari ni naru */
			for (; j < log_entry; j++) {
				result[logdata[j]] = 1;
			}
		}
	}
	for (i = 0; i < N; i++) {
		if (result[i] == 1) count++;
	}
	printf("%d\n", count);
	return 0;
}