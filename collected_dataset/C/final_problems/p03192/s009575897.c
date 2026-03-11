#include<stdio.h>

int main()
{
    long long int N;
    int count = 0;
    fscanf(stdin, "%lld", &N);

    do {
        if( (N % 10) == 2) {
            count++;
        }
    } while( (N = N/10) != 0);

    fprintf(stdout, "%d\n", count);

    return 0;
}