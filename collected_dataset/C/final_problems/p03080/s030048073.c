/*
 * main.c
 *
 *  Created on: 2019/03/30
 *      Author: family
 */


#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

char s[101] = {0};

int main()
{
    int N;
    int i = 0;
    int red = 0;
    scanf("%d", &N);
    scanf("%s", s);

    for(i = 0; i < N; i++) {
        if(s[i] == 'R') {
            red++;
        }
    }
    if((N - red) < red) {
        printf("Yes\n");
    } else {
        printf("No\n");
    }
    return 0;
}
