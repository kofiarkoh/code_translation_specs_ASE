#include <stdio.h>
#include <stdlib.h>

int main()
{
    int N;
    int K;
    int list[100000];
    int name[100000];
    int s;
    int i;
    int type;
    int index;
    int index2;
    int count;
    int zero_count;
    int max;
    int j;

    scanf("%d %d%*c", &N, &K);

    index = 0;
    s = getchar() - '0';
    type = s;
    if(type == 0) {
        zero_count = 1;
    } else {
        zero_count = 0;
    }
    count = 1;
    for(i = 1; i < N; i++) {
        s = getchar() - '0';
        if(s == type) {
            count += 1;
        } else {
            list[index] = count;
            name[index] = type;
            type = 1 - type;
            if(type == 0) {
                zero_count += 1;
            }
            index += 1;
            count = 1;
        }
    }
    list[index] = count;
    name[index] = type;
    if(type == 0) {
        zero_count += 1;
    }
    index += 1;

    if(zero_count <= K) {
        max = N;
    } else {
        max = 0;
        index2 = index - K;
        for(i = 0; i < index2; i++) {
            zero_count = 0;
            count = 0;
            for(j = i; j < index; j++) {
                count += list[j];
                if(name[j] == 0) {
                    zero_count += 1;
                } else if(zero_count == K) {
                    break;
                }
            }
            if(max < count) {
                max = count;
            }
        }
    }

    printf("%d\n", max);

    return 0;
}