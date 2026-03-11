#include <stdio.h>
typedef unsigned int uint32_t;
typedef int int32_t;
typedef long long int64_t;
#define NUM_CONTESTS (26)
#define MAX_DAY (365)


int main()
{
    int32_t D;
    int32_t c[NUM_CONTESTS];
    int32_t s[MAX_DAY][NUM_CONTESTS];
    int32_t t[MAX_DAY];
    int32_t score;
    
    // Read data
    scanf("%d", &D);
    for(int i =0; i<NUM_CONTESTS;i++){
        scanf("%d", &c[i]);
    }
    for(int i=0; i<D;i++){
        for(int j=0;j<NUM_CONTESTS;j++){
            scanf("%d", &s[i][j]);
        }
    }
    for(int i=0; i<D; i++){
        scanf("%d", t+i);
    }

    // Calc satisfaction score
    int M, d, q;
    scanf("%d", &M);
    for(int j=0; j<M;j++){
        scanf("%d %d", &d, &q);
        t[d-1] = q;

        score = 0;
        int last[NUM_CONTESTS]={0};
        for(int i=0; i<D;i++){
            int num = t[i]-1;
            score += s[i][num];
            last[num] = i+1;
            for(int j=0; j<NUM_CONTESTS; j++){
                score -= c[j]*(i+1-last[j]);
            }
        }
        printf("%d\n",score);
    }
    

    
    return 0;
}