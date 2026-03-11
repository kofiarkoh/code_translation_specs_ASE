#include <stdio.h>

typedef enum {false, true} bool;


bool trace(long long xy[][2], long long basis, long long xor_basis, int N, int num, int ctr, int index)
{
    int i, j;
    static int record[15];
    static bool flag;
    
    
    if(num == ctr)
    {
        flag = true;
        xor_basis = xor_basis ^ basis;
        
        for(i=0; i<num; i++)
        {
            if((xy[record[i]][1] & basis) != basis || ((xy[record[i]][0] ^ xor_basis) & xor_basis) != xor_basis)
            {
                flag = false;
                
                break;
            }
        }
    }
    else
    {
        for(j=index; j<=N-(num-ctr); j++)
        {
            if(ctr == 0)
                basis = 0;
            
            record[ctr] = j;
            
            if(trace(xy, basis | 1<<(N-j-1), xor_basis, N, num, ctr+1, j+1))
                return true;
        }
    }
    
    if(flag)
        return true;
    else
        return false;
}

int main(void)
{
    char c;
    int N;
    int A, x, y;
    
    long long xy[15][2], temp, temp2;
    
    int i, j;
    
    
    scanf("%d%c", &N, &c);
    
    
    for(i=0; i<N; i++)
    {
        scanf("%d%c", &A, &c);
        
        xy[i][0] = 1<<(N-i-1);
        
        xy[i][1] = 1;
        for(j=1; j<N; j++)
            xy[i][1] = xy[i][1]<<1 | 1;
        
        temp = xy[i][1];
        
        for(j=0; j<A; j++)
        {
            scanf("%d%c%d%c", &x, &c, &y, &c);
            
            if(y == 0)
            {
                temp2 = temp ^ 1<<(N-x);
                xy[i][1] &= temp2;
            }
            if(y == 1)
                xy[i][0] |= 1<<(N-x);
        }
    }
    
    for(i=N; i>0; i--)
    {
        if(trace(xy, 0, temp, N, i, 0, 0))
            break;
    }
    
    
    printf("%d\n", i);
    
    
    //system("pause");
    
    return 0;
}
