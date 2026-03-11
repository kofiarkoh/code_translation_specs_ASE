/*
 * main.c
 *
 *  Created on: 2019/04/20
 *      Author: family
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>


unsigned long long int ope(unsigned long long int* a)
{
    if(((*a)%2) == 1) {
        *a -= 1;
    }
    *a = *a/2;
    return (*a);
}

int main()
{
    unsigned long long int A,B;
    int K = 0;
    int i = 0;
    scanf("%llu %llu %d", &A, &B, &K);
    for(i = 0; i < K; i++) {
        if((i%2) == 0) {
            // takahashi
            B += ope(&A);
        } else {
            // aoki
            A += ope(&B);
        }
    }
    printf("%llu %llu\n", A, B);
    return 0;
}
