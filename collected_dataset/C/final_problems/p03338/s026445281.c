#include <stdio.h>

int main ()
{
    int N;
    scanf("%d", &N);

    char S[N];
    scanf("%s", S);

    int ans = 0;
    char X[N], Y[N];
    for (int i = 0; i < N; i++)
    {
        X[0] = '\0';
        Y[0] = '\0';
        int counter = 0;
        for (int j = 0; j <= i; j++)
        {
            X[j] = S[j];
        }
        for (int k = i; k < N; k++)
        {
            Y[k - i] = S[k + 1];
        }
        
        for (int L = 0; L < N; L++)
        {
            for (int M = L + 1; M < N; M++)
            {
                if (X[L] == X[M])
                {
                    X[M] = '\0';
                }
                if (Y[L] == Y[M])
                {
                    Y[M] = '\0';
                }
            }
        }

        for (int L = 0; L < N; L++)
        {
            for (int M = 0; M < N; M++)
            {
                if (X[L] == '\0' || Y[M] == '\0')
                {
                }
                else if (X[L] == Y[M])
                {
                    counter += 1;
                }
            }
        }

        if (counter > ans)
        {
            ans = counter;
        }
    }
    printf("%d", ans);
    return 0;
}