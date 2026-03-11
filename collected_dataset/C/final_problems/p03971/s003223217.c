#include <stdio.h>

int main(void)
{
    int N,A,B,i,kokunai = 0,kaigai = 0;
    char S[100002];
    scanf("%d %d %d\n%s",&N,&A,&B,S);
    for (i = 0; i < N; i++) {
        if (S[i] == 'a') {
            if (kokunai < (A + B)) {
                printf("Yes\n");
                kokunai++;
            } else {
                printf("No\n");
            }
        } else if (S[i] == 'b') {
            if (kokunai < (A + B) && kaigai < B) {
                printf("Yes\n");
                kokunai++;
                kaigai++;
            } else {
                printf("No\n");
            }
        } else {
            printf("No\n");
        }
    }
    return 0;
}
