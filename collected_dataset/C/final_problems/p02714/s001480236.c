#include <stdio.h>

int main(void)
{
    int N, answer = 0;
    char S[4000];
    scanf("%d", &N);
    scanf("%s", S);
    answer = N*(N-1)*(N-2)/6;

    for (int i = 0; i < N-2; i++)
    {
        for (int j = i+1; j < N-1; j++)
        {
            if (S[i] == S[j])
            {
                answer -= N-j-1;
                continue;
            }
            

            for (int k = j+1; k < N; k++)
            {
                if (S[i] == S[k] || S[j] == S[k] || (j-i) == (k-j))
                {
                    answer --;
                }
                
            }
            
        }
        
    }

    printf("%d", answer);
    
    return 0;
}