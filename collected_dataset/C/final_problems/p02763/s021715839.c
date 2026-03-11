#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int count(char *str, int start, int end) {
    int flag[26];
    for (int i = 0; i < 26; i++) {
        flag[i] = 0;
    }

    for (int i = start - 1; i < end; i++) {
        // printf("%c\n", str[i]);
        // printf("%d\n", (int) str[i] - 97);
        flag[(int) str[i] - 97] = 1;
    }

    int result = 0;
    for (int i = 0; i < 26; i++) {
        result += flag[i];
    }

    return result;
}

int main(void) {
    int length;
    scanf("%d", &length);
    char str[length];
    scanf(" %s", str);

    long int query_num;
    scanf(" %ld", &query_num);
    for (int i = 0; i < query_num; i++) {
        int type;
        scanf(" %d", &type);
        if (type == 1) {
            int num;
            char c;
            scanf(" %d %c", &num, &c);
            str[num - 1] = c;
            // printf("%s\n", str);
        } else if (type == 2) {
            int start, end;
            scanf(" %d %d", &start, &end);
            printf("%d\n", count(str, start, end));
        }
    }
    return 0;
}