#include <stdio.h>

typedef enum{false, true} bool;

int main(void)
{
    int T;
    int N;
    unsigned long long A[200];
    char S[201];
    char c;
    int i, j, k;
    
    int index;
    unsigned long long DP[50000];
    bool record[100];
    
    
    scanf("%d%c", &T, &c);
    for(i=0; i<T; i++)
    {
        scanf("%d%c", &N, &c);
        for(j=0; j<N; j++)
            scanf("%lld%c", &A[j], &c);
        scanf("%s", S);
        
        index = 0;
        DP[index++] = 0;
        record[i] = false;
        
        for(j=N-1; j>=0; j--)
        {
            if(S[j] == '0')
            {
                for(k=index-1; k>=0; k--)
                {
                    DP[index++] = DP[k] ^ A[j];
                }
            }
            else
            {
                for(k=0; k<index; k++)
                {
                    if(DP[k] == A[j])
                        break;
                }
                
                if(k == index)
                {
                    record[i] = true;
                    break;
                }
            }
        }
    }
    
    
    for(i=0; i<T; i++)
        printf("%d\n", record[i]);
    
    //system("pause");
    
    return 0;
}