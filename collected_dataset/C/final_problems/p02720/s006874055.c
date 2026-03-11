#include <stdio.h>

int main(void)
{
    unsigned long long int X[100000] = {0};
    unsigned long long int K, count = 0;
    int l = -1;
    scanf("%llu", &K);

    for (int i = 0; i < 9; i++)
    {
        X[i] = i+1;
    }

    for (unsigned long long int j = 9; j < K; j++)
    {   
        X[j] = X[count]*10 + (X[count]%10) + l;
        l++;

        if (X[count]%10 == 0)
        {
            X[j] ++;
            if (l == 1)
            {
                count++;
                l = -1;
            }
            
        }
        

        if (l == 1 && X[count]%10 == 9)
        {
            l = -1;
            count++;
        }
        
        if (l==2)
        {
            l = -1;
            count++;
        }

                
    }
    
    printf("%llu", X[K-1]);
    
    return 0;
}