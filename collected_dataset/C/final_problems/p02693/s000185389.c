/**

author:	©LoneWolfDON
Git: MahinHossainMunna

**/
/*

*/
#include <stdio.h>

int main()
{
    int K, A, B;

    scanf("%d%d%d", &K, &A, &B);

    int multiple = (B / K) * K;

    if (A <= multiple)
        printf("OK\n");

    else
        printf("NG\n");

    return 0;
 }
