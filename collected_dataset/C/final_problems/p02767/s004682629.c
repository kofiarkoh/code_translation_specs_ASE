#include <stdio.h>

int main(int argc, char const *argv[])
{
    int N;
    int X[100];
    int sum = 0;

    scanf("%d", &N);
    for(int i=0;i<N;i++)
    {
        scanf("%d", &X[i]);
        sum += X[i];
    }

    int avg = sum / N;
    avg = ((sum % N) < N - (sum % N)) ? avg : avg + 1;
    
    sum = 0;
    for (int i = 0; i < N; i++)
    {
        sum += (X[i]-avg)*(X[i]-avg);   
    }

    printf("%d", sum);

    return 0;
}
