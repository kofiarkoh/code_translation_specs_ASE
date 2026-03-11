#include <stdio.h>
#include <string.h>

int main() {
    int N, M, i, j, lock;
    char U[11][257];
    char s[11];

    scanf("%d", &N);

    for (i = 0; i < N; i++) {
        scanf("%s", U[i]);
    }

    scanf("%d", &M);

    lock = 1;

    for (i = 0; i < M; i++) {
        scanf("%s", s);
        for (j = 0; j < N; j++) {
            if (strcmp(s, U[j]) == 0 && lock) {
                printf("Opened by %s\n", s);
                lock = 0;
                break;
            }
            else if (strcmp(s, U[j]) == 0 && !lock) {
                printf("Closed by %s\n", s);
                lock = 1;
                break;
            }
            if (j == N-1) {
                printf("Unknown %s\n", s);
            }
        }
    }
    return 0;
}
